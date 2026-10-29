//! Pressmark OpenAPI 3.1 Specification & ChatGPT Plugin Manifest
//!
//! Author: Daniel Hallman

use serde_json::{json, Value};

/// ChatGPT Plugin Manifest (`/.well-known/ai-plugin.json`)
pub fn get_ai_plugin_manifest(server_url: &str) -> Value {
    let base = if server_url.starts_with("https://") {
        server_url
    } else {
        "https://pressmark.dev"
    };

    json!({
        "schema_version": "v1",
        "name_for_human": "Pressmark Document & Invoice Generator",
        "name_for_model": "pressmark_doc_engine",
        "description_for_human": "Create PDF invoices, estimates, receipts, contracts, non-disclosure agreements, and purchase orders. Add your branding, chosen notices, and signatures.",
        "description_for_model": "Creates and previews PDF documents with the supplied parties, line items, terms, logos, signatures, and notices. Use for requests to create, design, or download invoices, estimates, quotes, receipts, agreements, or contracts. Check the preview and required details before returning the document.",
        "auth": {
            "type": "oauth",
            "client_url": "https://auth.openai.com/api/accounts/authorize",
            "scope": "openid profile email model.request",
            "authorization_url": "https://auth.openai.com/api/accounts/oauth/token",
            "authorization_content_type": "application/json",
            "verification_tokens": {
                "openai": "pressmark_openai_action_verification_token"
            }
        },
        "api": {
            "type": "openapi",
            "url": format!("{}/api/v1/openapi.json", base)
        },
        "logo_url": format!("{}/assets/logo.jpg", base),
        "contact_email": "daniel@mooshworks.com",
        "legal_info_url": format!("{}/privacy", base)
    })
}

/// OpenAPI 3.1.0 Specification for ChatGPT Actions & Custom GPTs
pub fn get_openapi_spec(server_url: &str) -> Value {
    json!({
        "openapi": "3.1.0",
        "info": {
            "title": "Pressmark Vector Document Engine API",
            "description": "Creates PDF invoices, estimates, receipts, contracts, non-disclosure agreements, and certificates, with PNG previews.",
            "version": crate::mcp::MCP_SERVER_VERSION
        },
        "servers": [
            {
                "url": if server_url.starts_with("https://") {
                    server_url
                } else {
                    "https://pressmark.dev"
                },
                "description": "Production Pressmark Server"
            }
        ],
        "paths": {
            "/api/v1/generate": {
                "post": {
                    "operationId": "generateDocument",
                    "summary": "Generate a PDF document",
                    "description": "Creates a PDF and returns Base64 PDF data. Configured customer sync can save contact details and a document summary and start follow-up. See https://pressmark.dev/privacy.",
                    "security": [],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/DocumentRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Generated PDF document",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/GenerateResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/preview": {
                "post": {
                    "operationId": "previewDocument",
                    "summary": "Preview a document as PNG image",
                    "description": "Returns a PNG preview that can be displayed in chat.",
                    "security": [],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/DocumentRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Preview PNG image Base64 Data URI",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/PreviewResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/pricing": {
                "get": {
                    "operationId": "getPricing",
                    "summary": "Get subscription tiers and single pass prices",
                    "description": "Returns plans, prices, document limits, and checkout links.",
                    "security": [],
                    "responses": {
                        "200": {
                            "description": "Plans and prices"
                        }
                    }
                }
            },
            "/api/v1/lifecycle/convert": {
                "post": {
                    "operationId": "convertDocument",
                    "summary": "Convert document lifecycle state",
                    "description": "Transforms an estimate or quote into an invoice, receipt, or credit note, updating document numbering, issue/due dates, and payment states.",
                    "security": [],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/LifecycleConvertRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Converted document object"
                        }
                    }
                }
            },
            "/api/v1/lifecycle/pay": {
                "post": {
                    "operationId": "recordPayment",
                    "summary": "Record document payment",
                    "description": "Records full or partial payment against an invoice, recalculating remaining balance due, updating payment status, and issuing an official payment receipt.",
                    "security": [],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/LifecyclePayRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Updated document with calculated balance_due"
                        }
                    }
                }
            },
            "/api/v1/auth/chatgpt/session": {
                "get": {
                    "operationId": "getChatGptSession",
                    "summary": "Check ChatGPT login session and subscription plan",
                    "description": "Verifies whether the caller has authenticated via Sign in with ChatGPT and returns active subscription plan (Plus, Pro, Team, Enterprise) and compute allowance.",
                    "responses": {
                        "200": {
                            "description": "Session status, user identity, and plan details"
                        }
                    }
                }
            },
            "/api/v1/copilot": {
                "post": {
                    "operationId": "runCopilot",
                    "summary": "AI Copilot document editing",
                    "description": "Applies supported edits to document details from written instructions.",
                    "responses": {
                        "200": {
                            "description": "Modified document state and AI explanation"
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "DocumentRequest": {
                    "type": "object",
                    "properties": {
                        "doc_type": {
                            "type": "string",
                            "enum": crate::mcp::McpHandler::document_type_enum(),
                            "description": "The type of document to create"
                        },
                        "title": { "type": "string", "description": "Custom document title (e.g. 'INVOICE')" },
                        "doc_number": { "type": "string", "description": "Unique document or invoice number" },
                        "issue_date": { "type": "string", "description": "Document issue date (YYYY-MM-DD)" },
                        "due_date": { "type": "string", "description": "Payment due date (YYYY-MM-DD)" },
                        "theme": {
                            "type": "string",
                            "enum": ["modern_slate", "executive_navy", "emerald", "crimson_legal", "warm_sand", "obsidian"],
                            "description": "Preset visual styling theme"
                        },
                        "primary_color": { "type": "string", "description": "Custom primary hex color (e.g. #4f46e5)" },
                        "accent_color": { "type": "string", "description": "Custom accent hex color" },
                        "font_family": { "type": "string", "enum": ["sans", "serif", "mono"] },
                        "font_scale": { "type": "string", "enum": ["compact", "standard", "large"] },
                        "paper_size": { "type": "string", "enum": ["us-letter", "a4", "us-legal"] },
                        "signer_name": { "type": "string", "description": "Name of authorized signatory" },
                        "signer_title": { "type": "string", "description": "Title of authorized signatory" },
                        "from": {
                            "type": "object",
                            "required": ["name"],
                            "properties": {
                                "name": { "type": "string" },
                                "company": { "type": "string" },
                                "address": { "type": "string" },
                                "email": { "type": "string" },
                                "phone": { "type": "string" },
                                "tax_id": { "type": "string" }
                            }
                        },
                        "to": {
                            "type": "object",
                            "required": ["name"],
                            "properties": {
                                "name": { "type": "string" },
                                "company": { "type": "string" },
                                "address": { "type": "string" },
                                "email": { "type": "string" },
                                "phone": { "type": "string" },
                                "tax_id": { "type": "string" }
                            }
                        },
                        "items": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["description", "quantity", "unit_price"],
                                "properties": {
                                    "description": { "type": "string" },
                                    "quantity": { "type": "number" },
                                    "unit_price": { "type": "number" },
                                    "discount": { "type": "number" }
                                }
                            }
                        },
                        "currency": { "type": "string", "default": "USD" },
                        "tax_rate": { "type": "number", "description": "Percentage tax rate (e.g. 8.5 for 8.5%)" },
                        "discount_amount": { "type": "number" },
                        "notes": { "type": "string" },
                        "terms": { "type": "string" },
                        "payment_link": { "type": "string", "description": "Stripe checkout URL or custom payment link" },
                        "payment_qr_code": { "type": "boolean", "description": "Render embedded vector QR code for mobile payments" },
                        "payment_status": { "type": "string", "enum": ["unpaid", "pending", "partial", "paid", "refunded"] },
                        "payment_method": { "type": "string" },
                        "amount_paid": { "type": "number" },
                        "is_paid": { "type": "boolean" },
                        "watermark": { "type": "string" },
                        "state_code": { "type": "string", "description": "2-letter US state or DC jurisdiction code (e.g. TN, CA, NY, FL, TX, VA, MN, WI)" },
                        "municipality": { "type": "string", "description": "Municipal sub-profile override (Chicago, Cook County, New York City, San Francisco, Seattle, Portland)" },
                        "county": { "type": "string", "description": "County name for notarial jurat and deed recording headers" },
                        "recordable": { "type": "boolean", "description": "Enable 3.0-inch County Register of Deeds recording cover header" },
                        "notary_required": { "type": "boolean", "description": "Render state-specific Notarial Acknowledgment & Jurat block" },
                        "year_built": { "type": "integer", "description": "Property construction year (<1978 triggers Federal Lead-Based Paint disclosure)" },
                        "lease_package_mode": { "type": "boolean", "description": "Append mandatory standalone state attachment sheets and move-in checklists" },
                        "execution_mode": { "type": "string", "enum": ["esign", "wet_ink", "wet-ink", "notarized_wet", "ron", "notarized_ron"] },
                        "compliance_mode": { "type": "string", "enum": ["none", "eu_vat", "factur-x", "peppol_bis", "hipaa_baa", "zatca", "irs_w9", "pci_dss", "soc2_badge", "statutory_sworn"] },
                        "include_clerk_stamp": { "type": "boolean" },
                        "currency_symbol_mode": { "type": "string", "enum": ["symbol", "code"] },
                        "currency_placement": { "type": "string", "enum": ["prefix", "suffix"] },
                        "currency_separators": { "type": "string", "enum": ["comma_dot", "dot_comma", "space_comma", "apostrophe_dot", "plain"] },
                        "date_format": { "type": "string", "enum": ["iso", "us_slash", "euro", "formal_prose", "short_abbrev", "short_dot"] },
                        "number_decimals": { "type": "integer", "default": 2 },
                        "unit_measure": { "type": "string" },
                        "custom_fields": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["label", "value"],
                                "properties": {
                                    "label": { "type": "string" },
                                    "value": { "type": "string" }
                                }
                            }
                        },
                        "sections": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["title", "content"],
                                "properties": {
                                    "title": { "type": "string" },
                                    "content": { "type": "string" }
                                }
                            }
                        },
                        "signatures": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["signer_name"],
                                "properties": {
                                    "signer_name": { "type": "string" },
                                    "signer_title": { "type": "string" },
                                    "date": { "type": "string" }
                                }
                            }
                        }
                    },
                    "required": ["doc_type", "from", "to"]
                },
                "LifecycleConvertRequest": {
                    "type": "object",
                    "required": ["document", "target_type"],
                    "properties": {
                        "target_type": {
                            "type": "string",
                            "enum": ["invoice", "receipt", "estimate", "credit_note", "refund"],
                            "description": "Target lifecycle document type"
                        },
                        "due_days": { "type": "integer", "description": "Number of days until payment is due (for invoice conversion)" },
                        "valid_days": { "type": "integer", "description": "Number of days proposal is valid (for estimate conversion)" },
                        "payment_link": { "type": "string", "description": "Stripe checkout URL for online payments" },
                        "payment_qr_code": { "type": "boolean", "description": "Whether to include a QR code for the payment link" },
                        "credit_reason": { "type": "string", "description": "Reason for issuing credit note or refund" },
                        "document": { "$ref": "#/components/schemas/DocumentRequest" }
                    }
                },
                "LifecyclePayRequest": {
                    "type": "object",
                    "required": ["document"],
                    "properties": {
                        "document": { "$ref": "#/components/schemas/DocumentRequest" },
                        "payment_method": { "type": "string", "description": "Payment method (e.g. Stripe, Apple Pay, Bank Wire)" },
                        "transaction_reference": { "type": "string", "description": "Transaction reference or checkout ID" },
                        "paid_date": { "type": "string", "description": "Payment date (YYYY-MM-DD)" },
                        "amount_paid": { "type": "number", "description": "Amount received (if less than total, records partial payment)" }
                    }
                },
                "GenerateResponse": {
                    "type": "object",
                    "properties": {
                        "success": { "type": "boolean" },
                        "pdf_base64": { "type": "string", "description": "Raw Base64-encoded PDF bytes" },
                        "preview_data_uri": { "type": "string", "description": "Base64 PNG Data URI for visual rendering" },
                        "doc_number": { "type": "string" },
                        "grand_total": { "type": "number" },
                        "compile_time_ms": { "type": "integer" }
                    }
                },
                "PreviewResponse": {
                    "type": "object",
                    "properties": {
                        "success": { "type": "boolean" },
                        "image_data_uri": { "type": "string" },
                        "compile_time_ms": { "type": "integer" }
                    }
                }
            },
            "securitySchemes": {
                "chatgpt_oauth": {
                    "type": "oauth2",
                    "description": "Sign in with ChatGPT to delegate AI Copilot generation and OCR to your ChatGPT Plus/Pro/Team subscription quota.",
                    "flows": {
                        "authorizationCode": {
                            "authorizationUrl": "https://auth.openai.com/api/accounts/authorize",
                            "tokenUrl": "https://auth.openai.com/api/accounts/oauth/token",
                            "scopes": {
                                "openid": "OpenID Connect user profile",
                                "profile": "User profile information",
                                "email": "User email address",
                                "model.request": "Pass-through ChatGPT subscription compute allowance"
                            }
                        }
                    }
                },
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT",
                    "description": "ChatGPT user delegation session token"
                }
            }
        },
        "security": [
            { "chatgpt_oauth": ["openid", "profile", "email", "model.request"] },
            { "bearerAuth": [] },
            {}
        ]
    })
}
