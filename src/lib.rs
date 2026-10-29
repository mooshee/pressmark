//! Pressmark - High-Performance Vector Document Synthesis Engine
//!
//! Author: Daniel Hallman

#![recursion_limit = "512"]

pub mod auth;
pub mod branding;
pub mod builder;
pub mod downloads;
pub mod engine;
pub mod ghl;
pub mod mcp;
pub mod models;
pub mod openapi;
pub mod pricing;
pub mod rate_limit;
pub mod server;
pub mod state_laws;
pub mod storage;
pub mod stripe;
pub mod templates;

pub use auth::{ChatGptAuthManager, ChatGptPlan, ChatGptSession, ChatGptUser};
pub use branding::{BrandingMode, DocumentBrandingConfig};
pub use builder::{ExecutionMode, PressmarkDocumentBuilder};
pub use engine::{generate_pdf, generate_preview_png};
pub use models::DocumentRequest;
pub use server::{create_router, AppState};
