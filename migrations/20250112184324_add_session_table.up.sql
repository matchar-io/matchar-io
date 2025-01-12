CREATE TABLE
    "session" (
        "session_id" UUID,
        "user_id" UUID NOT NULL,
        "payload" JSONB NOT NULL,
        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "expired_at" TIMESTAMP NOT NULL,
        PRIMARY KEY ("session_id"),
        FOREIGN KEY ("user_id") REFERENCES "user" ("user_id")
    );

CREATE INDEX "index__01945bd4-c995-777a-b8cf-94f8323ac009" ON "session" ("user_id");
