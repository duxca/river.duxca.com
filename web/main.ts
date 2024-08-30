import './style.css';
import xs from 'xstream';
import { Stream } from 'xstream';
import * as Cycle from '@cycle/run'
import { makeOpenLayersDriver } from './oldriver';
import type { OlCommand, OlSource, OlCommandAddWaypoint, Coordinate } from './oldriver';
import { makeGeoLocationDriver } from './geolocationdriver';
import type { GeoCommand, GeoSource } from './geolocationdriver';

async function listRiverWaypoints(o: { riverId: number, next?: number }): Promise<{ riverWaypoints: Array<{ riverWaypointId: number, name: string, latitude: number, longitude: number }>, next?: number }> {
  const { riverId, next } = o;
  const res = await fetch("https://litestream-sandbox-4h2uh5x4wa-an.a.run.app/api", {
    mode: "cors",
    method: "POST",
    credentials: "include",
    headers: {
      'Accept': 'application/json',
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      type: "ListRiverWaypoints",
      riverId,
      limit: 1000,
    })
  });
  const json = await res.json();
  console.log(json);
  return json;
}

async function listRivers(o: { next?: number }): Promise<{ rivers: Array<{ riverId: number, name: string }>, next?: number }> {
  const { } = o;
  const res = await fetch("https://litestream-sandbox-4h2uh5x4wa-an.a.run.app/api", {
    mode: "cors",
    method: "POST",
    credentials: "include",
    headers: {
      'Accept': 'application/json',
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      type: "ListRivers",
      limit: 1000,
    })
  });
  const json = await res.json();
  console.log(json);
  return json;
}

interface Sources {
  OL: OlSource;
  GEO: GeoSource;
}

interface Sinks {
  OL: Stream<OlCommand>;
  GEO: Stream<GeoCommand>;
}

function main(o: Sources): Sinks {
  const {
    OL: { clickGps$, clickMap$, clickAddWaypoint$ },
    GEO: { pos$ },
  } = o;

  const posChanged$ = pos$.fold((prev, curr) => {
    return [prev[1], curr];
  }, [{
    longtiude: 0,
    latitude: 0,
    accuracy: 0
  }, {
    longtiude: 0,
    latitude: 0,
    accuracy: 0
  }]).filter(([a, b]) => a.accuracy !== b.accuracy || a.latitude !== b.latitude || a.longtiude !== b.longtiude).map(([_a, b]) => b);
  const ol$1: Stream<OlCommand> = xs.combine(clickGps$, posChanged$).map(
    ([_click, pos]): OlCommand => {
      return {
        type: "focus",
        longitude: pos.longtiude,
        latitude: pos.latitude,
      };
    }
  );
  const ol$2: Stream<OlCommand> = pos$.map(({
    longtiude,
    latitude,
    accuracy
  }): OlCommand => {
    return {
      type: "updateCurrentPosition",
      longtiude,
      latitude,
      accuracy
    };
  });
  const ol$3: Stream<OlCommand> = clickAddWaypoint$.map(({
    longitude,
    latitude
  }): OlCommand => {
    return {
      type: "addWaypoint",
      longitude,
      latitude,
    };
  });
  const ol$4 = clickMap$.filter((_) => false).map((_) => { throw new Error("unreachable") });
  // EPSG:4326 (WGS 84 の EPSGコード) 座標系

  const ol$5 = xs.fromPromise((async () => {
    const waypoints: Array<OlCommandAddWaypoint> = [];
    const { rivers } = await listRivers({});
    for (const river of rivers) {
      const { riverWaypoints } = await listRiverWaypoints({ riverId: river.riverId });
      for (const riverWaypoint of riverWaypoints) {
        waypoints.push({
          type: "addWaypoint",
          // name: riverWaypoint.name,
          longitude: riverWaypoint.longitude,
          latitude: riverWaypoint.latitude,
        });
      }
    }
    return xs.fromArray(waypoints);
  })()).flatten();
  return {
    OL: xs.merge(ol$1, ol$2, ol$3, ol$4, ol$5),
    GEO: xs.never(),
  };
}

Cycle.run(main as any, {
  GEO: makeGeoLocationDriver(),
  OL: makeOpenLayersDriver(),
} as any);
