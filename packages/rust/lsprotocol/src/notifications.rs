// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// ****** THIS IS A GENERATED FILE, DO NOT EDIT. ******
// Steps to generate:
// 1. Checkout https://github.com/microsoft/lsprotocol
// 2. Install nox: `python -m pip install nox`
// 3. Run command: `python -m nox --session build_lsp`

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

use crate::*;

pub trait Notification {
    type Params: serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static;
    const METHOD: &'static str;
}

pub struct WorkspaceDidChangeWorkspaceFoldersNotification;

impl Notification for WorkspaceDidChangeWorkspaceFoldersNotification {
    type Params = DidChangeWorkspaceFoldersParams;
    const METHOD: &'static str = "workspace/didChangeWorkspaceFolders";
}

pub struct WindowWorkDoneProgressCancelNotification;

impl Notification for WindowWorkDoneProgressCancelNotification {
    type Params = WorkDoneProgressCancelParams;
    const METHOD: &'static str = "window/workDoneProgress/cancel";
}

pub struct WorkspaceDidCreateFilesNotification;

impl Notification for WorkspaceDidCreateFilesNotification {
    type Params = CreateFilesParams;
    const METHOD: &'static str = "workspace/didCreateFiles";
}

pub struct WorkspaceDidRenameFilesNotification;

impl Notification for WorkspaceDidRenameFilesNotification {
    type Params = RenameFilesParams;
    const METHOD: &'static str = "workspace/didRenameFiles";
}

pub struct WorkspaceDidDeleteFilesNotification;

impl Notification for WorkspaceDidDeleteFilesNotification {
    type Params = DeleteFilesParams;
    const METHOD: &'static str = "workspace/didDeleteFiles";
}

pub struct NotebookDocumentDidOpenNotification;

impl Notification for NotebookDocumentDidOpenNotification {
    type Params = DidOpenNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didOpen";
}

pub struct NotebookDocumentDidChangeNotification;

impl Notification for NotebookDocumentDidChangeNotification {
    type Params = DidChangeNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didChange";
}

pub struct NotebookDocumentDidSaveNotification;

impl Notification for NotebookDocumentDidSaveNotification {
    type Params = DidSaveNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didSave";
}

pub struct NotebookDocumentDidCloseNotification;

impl Notification for NotebookDocumentDidCloseNotification {
    type Params = DidCloseNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didClose";
}

pub struct InitializedNotification;

impl Notification for InitializedNotification {
    type Params = InitializedParams;
    const METHOD: &'static str = "initialized";
}

pub struct ExitNotification;

impl Notification for ExitNotification {
    type Params = ();
    const METHOD: &'static str = "exit";
}

pub struct WorkspaceDidChangeConfigurationNotification;

impl Notification for WorkspaceDidChangeConfigurationNotification {
    type Params = DidChangeConfigurationParams;
    const METHOD: &'static str = "workspace/didChangeConfiguration";
}

pub struct WindowShowMessageNotification;

impl Notification for WindowShowMessageNotification {
    type Params = ShowMessageParams;
    const METHOD: &'static str = "window/showMessage";
}

pub struct WindowLogMessageNotification;

impl Notification for WindowLogMessageNotification {
    type Params = LogMessageParams;
    const METHOD: &'static str = "window/logMessage";
}

pub struct TelemetryEventNotification;

impl Notification for TelemetryEventNotification {
    type Params = LSPAny;
    const METHOD: &'static str = "telemetry/event";
}

pub struct TextDocumentDidOpenNotification;

impl Notification for TextDocumentDidOpenNotification {
    type Params = DidOpenTextDocumentParams;
    const METHOD: &'static str = "textDocument/didOpen";
}

pub struct TextDocumentDidChangeNotification;

impl Notification for TextDocumentDidChangeNotification {
    type Params = DidChangeTextDocumentParams;
    const METHOD: &'static str = "textDocument/didChange";
}

pub struct TextDocumentDidCloseNotification;

impl Notification for TextDocumentDidCloseNotification {
    type Params = DidCloseTextDocumentParams;
    const METHOD: &'static str = "textDocument/didClose";
}

pub struct TextDocumentDidSaveNotification;

impl Notification for TextDocumentDidSaveNotification {
    type Params = DidSaveTextDocumentParams;
    const METHOD: &'static str = "textDocument/didSave";
}

pub struct TextDocumentWillSaveNotification;

impl Notification for TextDocumentWillSaveNotification {
    type Params = WillSaveTextDocumentParams;
    const METHOD: &'static str = "textDocument/willSave";
}

pub struct WorkspaceDidChangeWatchedFilesNotification;

impl Notification for WorkspaceDidChangeWatchedFilesNotification {
    type Params = DidChangeWatchedFilesParams;
    const METHOD: &'static str = "workspace/didChangeWatchedFiles";
}

pub struct TextDocumentPublishDiagnosticsNotification;

impl Notification for TextDocumentPublishDiagnosticsNotification {
    type Params = PublishDiagnosticsParams;
    const METHOD: &'static str = "textDocument/publishDiagnostics";
}

pub struct SetTraceNotification;

impl Notification for SetTraceNotification {
    type Params = SetTraceParams;
    const METHOD: &'static str = "setTrace";
}

pub struct LogTraceNotification;

impl Notification for LogTraceNotification {
    type Params = LogTraceParams;
    const METHOD: &'static str = "logTrace";
}

pub struct CancelRequestNotification;

impl Notification for CancelRequestNotification {
    type Params = CancelParams;
    const METHOD: &'static str = "cancelRequest";
}

pub struct ProgressNotification;

impl Notification for ProgressNotification {
    type Params = ProgressParams;
    const METHOD: &'static str = "progress";
}
