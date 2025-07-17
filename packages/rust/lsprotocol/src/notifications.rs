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
/// The `workspace/didChangeWorkspaceFolders` notification is sent from the client to the server when the workspace
/// folder configuration changes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidChangeWorkspaceFoldersNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidChangeWorkspaceFoldersParams,
}

/// The `window/workDoneProgress/cancel` notification is sent from  the client to the server to cancel a progress
/// initiated on the server side.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkDoneProgressCancelNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: WorkDoneProgressCancelParams,
}

/// The did create files notification is sent from the client to the server when
/// files were created from within the client.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidCreateFilesNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: CreateFilesParams,
}

/// The did rename files notification is sent from the client to the server when
/// files were renamed from within the client.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidRenameFilesNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: RenameFilesParams,
}

/// The will delete files request is sent from the client to the server before files are actually
/// deleted as long as the deletion is triggered from within the client.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidDeleteFilesNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DeleteFilesParams,
}

/// A notification sent when a notebook opens.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidOpenNotebookDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidOpenNotebookDocumentParams,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidChangeNotebookDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidChangeNotebookDocumentParams,
}

/// A notification sent when a notebook document is saved.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidSaveNotebookDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidSaveNotebookDocumentParams,
}

/// A notification sent when a notebook closes.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidCloseNotebookDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidCloseNotebookDocumentParams,
}

/// The initialized notification is sent from the client to the
/// server after the client is fully initialized and the server
/// is allowed to send requests from the server to the client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InitializedNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: Option<LSPAny>,
}

/// The exit event is sent from the client to the server to
/// ask the server to exit its process.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExitNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: Option<()>,
}

/// The configuration change notification is sent from the client to the server
/// when the client's configuration has changed. The notification contains
/// the changed configuration as defined by the language client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidChangeConfigurationNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidChangeConfigurationParams,
}

/// The show message notification is sent from a server to a client to ask
/// the client to display a particular message in the user interface.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShowMessageNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: ShowMessageParams,
}

/// The log message notification is sent from the server to the client to ask
/// the client to log a particular message.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogMessageNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: LogMessageParams,
}

/// The telemetry event notification is sent from the server to the client to ask
/// the client to log telemetry data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TelemetryEventNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: Option<LSPAny>,
}

/// The document open notification is sent from the client to the server to signal
/// newly opened text documents. The document's truth is now managed by the client
/// and the server must not try to read the document's truth using the document's
/// uri. Open in this sense means it is managed by the client. It doesn't necessarily
/// mean that its content is presented in an editor. An open notification must not
/// be sent more than once without a corresponding close notification send before.
/// This means open and close notification must be balanced and the max open count
/// is one.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidOpenTextDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidOpenTextDocumentParams,
}

/// The document change notification is sent from the client to the server to signal
/// changes to a text document.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidChangeTextDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidChangeTextDocumentParams,
}

/// The document close notification is sent from the client to the server when
/// the document got closed in the client. The document's truth now exists where
/// the document's uri points to (e.g. if the document's uri is a file uri the
/// truth now exists on disk). As with the open notification the close notification
/// is about managing the document's content. Receiving a close notification
/// doesn't mean that the document was open in an editor before. A close
/// notification requires a previous open notification to be sent.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidCloseTextDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidCloseTextDocumentParams,
}

/// The document save notification is sent from the client to the server when
/// the document got saved in the client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidSaveTextDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidSaveTextDocumentParams,
}

/// A document will save notification is sent from the client to the server before
/// the document is actually saved.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WillSaveTextDocumentNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: WillSaveTextDocumentParams,
}

/// The watched files notification is sent from the client to the server when
/// the client detects changes to file watched by the language client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DidChangeWatchedFilesNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: DidChangeWatchedFilesParams,
}

/// Diagnostics notification are sent from the server to the client to signal
/// results of validation runs.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublishDiagnosticsNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: PublishDiagnosticsParams,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetTraceNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: SetTraceParams,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogTraceNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: LogTraceParams,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CancelNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: CancelParams,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProgressNotification {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPNotificationMethods,

    pub params: ProgressParams,
}
