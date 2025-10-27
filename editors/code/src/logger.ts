import { inspect } from 'node:util';
import * as vscode from 'vscode';
const output = vscode.window.createOutputChannel('SurrealDB Extension', {log:true});

const stringify = (messages: [unknown, ...unknown[]]) => {
  return messages.map(message => {
    if (typeof message === 'string') {
      return message;
    }
    if (message instanceof Error) {
      return message.stack;
    }
    return inspect(message);
  }).join(' ');
}

export const logger = {
  info: (...messages: [unknown, ...unknown[]]) => {
    output.info(stringify(messages));
  },
  error: (...messages: [unknown, ...unknown[]]) => {
    output.error(stringify(messages));
  },
  warn: (...messages: [unknown, ...unknown[]]) => {
    output.warn(stringify(messages));
  },
  debug: (...messages: [unknown, ...unknown[]]) => {
    output.debug(stringify(messages));
  },
  trace: (...messages: [unknown, ...unknown[]]) => {
    output.trace(stringify(messages));
  },
}