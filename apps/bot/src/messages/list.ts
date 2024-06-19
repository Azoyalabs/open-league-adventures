export const builder = (header: string, content: string[]) => {
  return `**${header}**\n${content.join("\n")}`;
};
