ALTER TABLE alerts
    ADD COLUMN status          TEXT  NOT NULL DEFAULT 'open',
    ADD COLUMN assignee        TEXT,
    ADD COLUMN description     TEXT,
    ADD COLUMN mitre_technique TEXT,
    ADD COLUMN entities        JSONB NOT NULL DEFAULT '[]',
    ADD COLUMN activity        JSONB NOT NULL DEFAULT '[]',
    ADD COLUMN events          JSONB NOT NULL DEFAULT '[]';