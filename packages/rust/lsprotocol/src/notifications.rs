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
    const METHOD: &'static str = "WorkspaceDidChangeWorkspaceFolders";
}

pub struct WindowWorkDoneProgressCancelNotification;

impl Notification for WindowWorkDoneProgressCancelNotification {
    type Params = WorkDoneProgressCancelParams;
    const METHOD: &'static str = "WindowWorkDoneProgressCancel";
}

pub struct WorkspaceDidCreateFilesNotification;

impl Notification for WorkspaceDidCreateFilesNotification {
    type Params = CreateFilesParams;
    const METHOD: &'static str = "WorkspaceDidCreateFiles";
}

pub struct WorkspaceDidRenameFilesNotification;

impl Notification for WorkspaceDidRenameFilesNotification {
    type Params = RenameFilesParams;
    const METHOD: &'static str = "WorkspaceDidRenameFiles";
}

pub struct WorkspaceDidDeleteFilesNotification;

impl Notification for WorkspaceDidDeleteFilesNotification {
    type Params = DeleteFilesParams;
    const METHOD: &'static str = "WorkspaceDidDeleteFiles";
}

pub struct NotebookDocumentDidOpenNotification;

impl Notification for NotebookDocumentDidOpenNotification {
    type Params = DidOpenNotebookDocumentParams;
    const METHOD: &'static str = "NotebookDocumentDidOpen";
}

pub struct NotebookDocumentDidChangeNotification;

impl Notification for NotebookDocumentDidChangeNotification {
    type Params = DidChangeNotebookDocumentParams;
    const METHOD: &'static str = "NotebookDocumentDidChange";
}

pub struct NotebookDocumentDidSaveNotification;

impl Notification for NotebookDocumentDidSaveNotification {
    type Params = DidSaveNotebookDocumentParams;
    const METHOD: &'static str = "NotebookDocumentDidSave";
}

pub struct NotebookDocumentDidCloseNotification;

impl Notification for NotebookDocumentDidCloseNotification {
    type Params = DidCloseNotebookDocumentParams;
    const METHOD: &'static str = "NotebookDocumentDidClose";
}

pub struct InitializedNotification;

impl Notification for InitializedNotification {
    type Params = InitializedParams;
    const METHOD: &'static str = "Initialized";
}

pub struct ExitNotification;

impl Notification for ExitNotification {
    type Params = ();
    const METHOD: &'static str = "Exit";
}

pub struct WorkspaceDidChangeConfigurationNotification;

impl Notification for WorkspaceDidChangeConfigurationNotification {
    type Params = DidChangeConfigurationParams;
    const METHOD: &'static str = "WorkspaceDidChangeConfiguration";
}

pub struct WindowShowMessageNotification;

impl Notification for WindowShowMessageNotification {
    type Params = ShowMessageParams;
    const METHOD: &'static str = "WindowShowMessage";
}

pub struct WindowLogMessageNotification;

impl Notification for WindowLogMessageNotification {
    type Params = LogMessageParams;
    const METHOD: &'static str = "WindowLogMessage";
}

pub struct TelemetryEventNotification;

impl Notification for TelemetryEventNotification {
    type Params = LSPAny;
    const METHOD: &'static str = "TelemetryEvent";
}

pub struct TextDocumentDidOpenNotification;

impl Notification for TextDocumentDidOpenNotification {
    type Params = DidOpenTextDocumentParams;
    const METHOD: &'static str = "TextDocumentDidOpen";
}

pub struct TextDocumentDidChangeNotification;

impl Notification for TextDocumentDidChangeNotification {
    type Params = DidChangeTextDocumentParams;
    const METHOD: &'static str = "TextDocumentDidChange";
}

pub struct TextDocumentDidCloseNotification;

impl Notification for TextDocumentDidCloseNotification {
    type Params = DidCloseTextDocumentParams;
    const METHOD: &'static str = "TextDocumentDidClose";
}

pub struct TextDocumentDidSaveNotification;

impl Notification for TextDocumentDidSaveNotification {
    type Params = DidSaveTextDocumentParams;
    const METHOD: &'static str = "TextDocumentDidSave";
}

pub struct TextDocumentWillSaveNotification;

impl Notification for TextDocumentWillSaveNotification {
    type Params = WillSaveTextDocumentParams;
    const METHOD: &'static str = "TextDocumentWillSave";
}

pub struct WorkspaceDidChangeWatchedFilesNotification;

impl Notification for WorkspaceDidChangeWatchedFilesNotification {
    type Params = DidChangeWatchedFilesParams;
    const METHOD: &'static str = "WorkspaceDidChangeWatchedFiles";
}

pub struct TextDocumentPublishDiagnosticsNotification;

impl Notification for TextDocumentPublishDiagnosticsNotification {
    type Params = PublishDiagnosticsParams;
    const METHOD: &'static str = "TextDocumentPublishDiagnostics";
}

pub struct SetTraceNotification;

impl Notification for SetTraceNotification {
    type Params = SetTraceParams;
    const METHOD: &'static str = "SetTrace";
}

pub struct LogTraceNotification;

impl Notification for LogTraceNotification {
    type Params = LogTraceParams;
    const METHOD: &'static str = "LogTrace";
}

pub struct CancelRequestNotification;

impl Notification for CancelRequestNotification {
    type Params = CancelParams;
    const METHOD: &'static str = "CancelRequest";
}

pub struct ProgressNotification;

impl Notification for ProgressNotification {
    type Params = ProgressParams;
    const METHOD: &'static str = "Progress";
}
