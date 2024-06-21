import "dotenv/config";

import { Telegraf, Markup } from "telegraf";
import { GAME_URL } from "./constants";

import { createClient } from "@supabase/supabase-js";
import { type Database } from "./types/supabase";
import { START_REPLY } from "./messages/start";
import { builder } from "./messages/list";
import { escapeMarkdown } from "./messages/escaper";

const supabase = createClient<Database>(
  process.env.PUBLIC_SUPABASE_URL!,
  process.env.SUPABASE_KEY!
);

async function main() {
  const bot = new Telegraf(process.env.TELEGRAM_DEV_HTTP_TOKEN!);

  bot.telegram.setMyCommands([
    {
      command: "/start",
      description: "Start Playing",
    },
    {
      command: "/help",
      description: "Get help",
    },
    {
      command: "/top_players",
      description: "Get the current top players",
    },
    {
      command: "/top_clans",
      description: "Get the current top clans",
    },
    {
      command: "/create_clan",
      description:
        "Create a new clan with the given name: /create_clan <name>",
    },
  ]);

  bot.command("start", async (ctx) => {
    const playerID = ctx.chat.id;
    const chat = await ctx.telegram.getChat(playerID);
    const { data, error } = await supabase
      .from("player")
      .insert({ id: playerID.toString() })
      .select();

    if (error === null) {
      const { data: count } = await supabase.rpc("initialize_account", {
        playerid: playerID.toString(),
      });

      console.log(`Created ${count} characters`);
    } else {
      console.log(`User already has an account: ${ctx.chat.id}`);
    }

    return ctx.replyWithMarkdownV2(escapeMarkdown(START_REPLY), {
      ...Markup.inlineKeyboard([
        Markup.button.url("Start Playing 🕹️", GAME_URL),
      ]),
    });
  });

  bot.help(async (ctx) => {
    const commands = await ctx.getMyCommands();
    const info = commands.reduce(
      (acc, val) => `${acc}/${val.command} - ${val.description}\n`,
      ""
    );
    return ctx.reply(info);
  });

  bot.hears(/^\/create_clan (.*)/, async (ctx) => {
    const reg = /create_clan\s?(.*)/;
    const executed = reg.exec(ctx.message.text);

    if (executed && executed.index === 1) {
      const clanName = executed[1];

      const { data } = await supabase
        .from("player")
        .select("clanid")
        .eq("id", ctx.chat.id)
        .single();

      if (data && data?.clanid) {
        return ctx.reply("You're already part of a clan");
      }

      const message = await ctx.reply(
        `Create a clan with the name ${clanName}?`,
        {
          reply_markup: {
            inline_keyboard: [
              [
                { text: "Yes", callback_data: `create_clan::${clanName}` },
                { text: "No", callback_data: "abort" },
              ],
            ],
          },
        }
      );

      bot.on("callback_query", async (cb_ctx) => {
        // @ts-expect-error we actually are expecting data in this case
        const data = cb_ctx.callbackQuery.data as string;
        if (data === "abort") {
          await cb_ctx.telegram.deleteMessage(
            message.chat.id,
            message.message_id
          );
        } else {
          const created = await supabase
            .from("clan")
            .insert({ clanname: clanName, blason: "", clandescription: "" })
            .select()
            .single();

          if (created.error) {
            cb_ctx.reply("Couldn't create clan");
          } else {
            await supabase
              .from("player")
              .update({ clanid: created.data.id })
              .eq("id", ctx.chat.id);

            cb_ctx.reply(`Clan ${clanName} successfully created`);
          }
        }
      });
    } else {
      ctx.reply("Couldn't create a clan");
    }
  });

  bot.command("top_players", async (ctx) => {
    const { data: players } = await supabase
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
