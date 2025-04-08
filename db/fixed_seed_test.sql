-- add admin user
INSERT INTO users (nickname, role) VALUES ('legokichi', 0);
INSERT INTO user_auths (user_id, identity_type, identifier) VALUES (1, 0, '2429307'); -- github

-- 修正：user_idとdescriptionを追加
INSERT INTO rivers (user_id, river_name, waypoint, description) VALUES (1, '荒川秩父', json_array(36.002780508652975,139.0719155531588), '');
INSERT INTO river_waypoints (river_id, user_id, waypoint_name, waypoint, description) VALUES (1, 1, '秩父公園橋', json_array(36.002780508652975,139.0719155531588), '');
INSERT INTO river_waypoints (river_id, user_id, waypoint_name, waypoint, description) VALUES (1, 1, '武之鼻橋', json_array(36.003201622651204,139.0725162723582), '');