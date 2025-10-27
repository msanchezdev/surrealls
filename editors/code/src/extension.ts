import * as vscode from 'vscode'	
import * as path from 'node:path';
import { type Executable, LanguageClient, ServerOptions } from 'vscode-languageclient/node';
import { logger } from './logger';

let lc: LanguageClient;

export async function activate(context: vscode.ExtensionContext) {
  logger.info("Activating SurrealDB Language Server extension");
  let serverExecutable = context.asAbsolutePath(path.join('..', '..', 'target', 'debug', 'surrealls'));
  const exec: Executable = {
    command: serverExecutable,
    args: ["lsp"],
    options: {
      env: {}
    }
  }
  let serverOptions: ServerOptions = {
    run: exec,
    debug: exec,
  }
  lc = new LanguageClient("surrealls", "SurrealDB Language Server", serverOptions, {
    documentSelector: [
      { language: "surrealql" },
      { language: "plaintext" },
    ]
  });
  await lc.start();
}

export async function deactivate() {
  await lc?.stop();
}