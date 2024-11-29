-- Task Kinds ---------------------------------------------------------------------
-- NOTE: This is defined here because it is used in both workers and tasks tables.

-- Each task has a "kind" which describes that class of task
CREATE TABLE task_kinds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Workers ------------------------------------------------------------------------

-- Workers execute tasks and send heartbeats to the server to indicate that they are still alive
CREATE TABLE workers (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    registered_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Mapping between workers and the tasks they can execute
CREATE TABLE worker_task_kinds (
    worker_id UUID NOT NULL REFERENCES workers(id),
    task_kind_id UUID NOT NULL REFERENCES task_kinds(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (worker_id, task_kind_id)
);

-- Heartbeats are regularly sent by the workers to indicate that they are still alive and kicking
CREATE TABLE worker_heartbeats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    worker_id UUID NOT NULL REFERENCES workers(id) ON DELETE CASCADE,
    heartbeat_time TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Tasks --------------------------------------------------------------------------

-- Task status enum
-- NOTE: This is currently not used because it's not easy to integrate with sqlx. Will come back to it.
CREATE TYPE task_status AS ENUM (
    'pending',    -- Task is created but not yet assigned
    'queued',     -- Task has been assigned to a worker and sent to a queue
    'running',    -- Worker has started processing
    'completed',  -- Task completed successfully
    'failed',     -- Task failed to complete
    'cancelled'   -- Task was cancelled before completion
);

-- Tasks are the actual task "instances" that are created and sent to workers
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_kind_id UUID NOT NULL REFERENCES task_kinds(id),
    input_data JSONB,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    assigned_to UUID REFERENCES workers(id)
);

-- New task results table
CREATE TABLE task_results (
    task_id UUID PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    output_data JSONB,
    error_data JSONB,
    worker_id UUID NOT NULL REFERENCES workers(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);