CREATE TABLE question (
    id VARCHAR(16) PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    chapter SMALLINT
);

CREATE TABLE option (
    id VARCHAR(16) PRIMARY KEY,
    question_id VARCHAR(16) NOT NULL REFERENCES question(id),
    body TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE student (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identifier TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE (identifier)
);

CREATE TABLE answer (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES student(id),
    question_id VARCHAR(16) NOT NULL REFERENCES question(id),
    option_id VARCHAR(16) NOT NULL REFERENCES option(id),
    submitted_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE (student_id, question_id)
);
