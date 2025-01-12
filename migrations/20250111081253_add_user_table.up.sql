CREATE TABLE
    "user" (
        "user_id" UUID,
        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY ("user_id")
    );

CREATE TABLE
    "user_profile" (
        "user_id" UUID,
        "name" VARCHAR(20) NOT NULL,
        "image_url" VARCHAR(255) NOT NULL,
        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY ("user_id"),
        FOREIGN KEY ("user_id") REFERENCES "user" ("user_id") ON DELETE NO ACTION
    );

CREATE UNIQUE INDEX "unique__01945471-4537-7bbd-86d3-ffc93c649454" ON "user_profile" ("name");

CREATE TABLE
    "identity_provider" (
        "identity_provider_id" UUID,
        "name" VARCHAR(20) NOT NULL,
        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY ("identity_provider_id")
    );

CREATE UNIQUE INDEX "unique__0194547c-9641-7117-be53-4e78163cf8a7" ON "identity_provider" ("name");

INSERT INTO
    "identity_provider" ("identity_provider_id", "name")
VALUES
    ('01945bc2-1786-7668-8b29-20f63e8c8e0f', 'Google');

CREATE TABLE
    "user_credential" (
        "user_id" UUID,
        "identity_provider_id" UUID,
        "external_id" VARCHAR(255) NOT NULL,
        "email_address" VARCHAR(255) NOT NULL,
        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY ("user_id"),
        FOREIGN KEY ("user_id") REFERENCES "user" ("user_id") ON DELETE NO ACTION,
        FOREIGN KEY ("identity_provider_id") REFERENCES "identity_provider" ("identity_provider_id") ON DELETE NO ACTION
    );

CREATE UNIQUE INDEX "unique__01945472-d63d-7bbc-811c-45ab37522d36" ON "user_credential" ("identity_provider_id", "external_id");
