import xs from 'xstream';
import type { Driver } from '@cycle/run'
import type { Stream } from 'xstream';

export interface RiverSource {
  listRiver$: Stream<{}>,
  listRiverWaypoints$: Stream<{}>,
}
export type RiverCommand = ListRiversCommand | ListRiverWaypointsCommand;
export interface ListRiversCommand {
  type: "ListRivers";
}
export interface ListRiverWaypointsCommand {
  type: "ListRiverWaypoints";
  riverId: number;
}

export function makeRiverDriver(): Driver<Stream<RiverCommand>, RiverSource> {
  function riverDriver(outgoing$: Stream<RiverCommand>) {
    outgoing$.addListener({
      next: (outgoing) => {
        switch (outgoing.type) {
          case "ListRivers": {
            listRivers({}).then((res) => {
              console.log(res);
            });
            break;
          }
          case "ListRiverWaypoints": {
            listRiverWaypoints({ riverId: outgoing.riverId }).then((res) => {
              console.log(res);
            });
            break
          }
        }
        console.log("next");
      },
      error: (err) => { console.error(err); },
      complete: () => { console.log("complete"); },
    });
    return {
      listRiver$,
      listRiverWaypoints$,
    };
  }
  return riverDriver;
}


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