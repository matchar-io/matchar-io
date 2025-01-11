CREATE TABLE
    "pkce" (
        "pkce_id" UUID,
        "csrf_token" VARCHAR(22) NOT NULL,
        "code_verifier" VARCHAR(43) NOT NULL,
        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "expired_at" TIMESTAMP NOT NULL,
        PRIMARY KEY ("pkce_id")
    );

CREATE INDEX "index__019453f3-3c20-7bb3-9507-9fa80fed2d00" ON "pkce" ("csrf_token");
