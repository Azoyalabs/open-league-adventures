import "dotenv/config";

import { Telegraf, Markup } from "telegraf";
import { GAME_URL } from "./constants";

import { createClient } from "@supabase/supabase-js";
import { type Database } from "./types/supabase";
import { START_REPLY } from "./messages/start";
import { builder } from "./messages/list";

const supabase = createClient<Database>(
  process.env.PUBLIC_SUPABASE_URL!,
  process.env.SUPABASE_KEY!
);

async function main() {
  const bot = new Telegraf(process.env.TELEGRAM_DEV_HTTP_TOKEN!);

  bot.command("start", async (ctx) => {
    const chat = await ctx.telegram.getChat(ctx.chat.id);
    await supabase.from("player").insert({ id: chat.id.toString() });

    // rpc call for creation

    return ctx.replyWithMarkdownV2(START_REPLY, {
      ...Markup.inlineKeyboard([
        Markup.button.url("Start Playing 🕹️", GAME_URL),
      ]),
    });
  });

  bot.command("top_players", async (ctx) => {
    const { data: players} = await supabase
      .from("playerpower")
      .select("*")
      .order("playerpower", { ascending: false })
      .limit(10);

    return ctx.replyWithMarkdownV2(
      builder(
        `\\Top 10 Clans`,
        (players || []).map((p, i) => {
          return `${i + 1}\\. ${p.playerid} \\- ${p.playerpower} power`;
        })
      )
    );
  });

  bot.command("top_clans", async (ctx) => {
    const { data: clans } = await supabase
      .from("clanpowerlevel")
      .select("*")
      .order("clanpower", { ascending: false })
      .limit(10);

    return ctx.replyWithMarkdownV2(
      builder(
        `\\Top 10 Clans`,
        (clans || []).map((c, i) => {
          return `${i + 1}\\. ${c.clanname} \\- ${c.clanpower} power`;
        })
      )
    );
  });

  console.log("STARTING BOT");

  await bot.launch();
  process.once("SIGINT", () => bot.stop("SIGINT"));
  process.once("SIGTERM", () => bot.stop("SIGTERM"));
}

main()
  .catch(console.error)
  .finally(() => process.exit(0));
