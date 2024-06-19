export const COMMANDS = ["start", "top_players", "top_clans"] as const;

export type Commands = (typeof COMMANDS)[number];
