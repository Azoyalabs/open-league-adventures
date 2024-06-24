// Generate typing for files

import ts from "typescript";
import * as fs from "node:fs";
import { resolve } from "node:path";

const printer = ts.createPrinter({ newLine: ts.NewLineKind.LineFeed });
const typeDirectory = resolve(__dirname, "../src/types");

const assetsDirectory = resolve(__dirname, "../src/assets");

function main() {
  const assetFolders = fs
    .readdirSync(assetsDirectory, { withFileTypes: true })
    .filter((f) => f.isDirectory());

  assetFolders.forEach((folder) => {
    const ouputFile = resolve(typeDirectory, `./${folder.name}.d.ts`);

    const filesInFolder = fs
      .readdirSync(resolve(folder.path, folder.name), { withFileTypes: true })
      .filter((f) => f.isFile() && !f.name.endsWith(".ts"));

    console.log("filesInFolder", folder.path, filesInFolder);

    const sourceFile = ts.createSourceFile(
      ouputFile,
      "",
      ts.ScriptTarget.ESNext,
      false,
      ts.ScriptKind.TS
    );

    const union = ts.factory.createUnionTypeNode(
      filesInFolder.map((f) =>
        ts.factory.createLiteralTypeNode(
          ts.factory.createStringLiteral(`./${f.name}`)
        )
      )
    );

    const assetsDeclaration = ts.factory.createTypeAliasDeclaration(
      [ts.factory.createModifier(ts.SyntaxKind.ExportKeyword)],
      ts.factory.createIdentifier(folder.name.toUpperCase()),
      undefined,
      union
    );

    const assetsOutput = printer.printNode(
      ts.EmitHint.Unspecified,
      assetsDeclaration,
      sourceFile
    );
    fs.writeFileSync(
      resolve(typeDirectory, `./${folder.name}.ts`),
      assetsOutput,
      {}
    );
  });
}

(() => {
  main();
})();
