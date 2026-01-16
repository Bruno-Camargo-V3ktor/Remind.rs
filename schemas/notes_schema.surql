DEFINE TABLE IF NOT EXISTS notes SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS id ON notes TYPE UUID;
DEFINE FIELD IF NOT EXISTS title ON notes TYPE string;
DEFINE FIELD IF NOT EXISTS content ON notes TYPE string;
DEFINE FIELD IF NOT EXISTS color ON notes TYPE string;
DEFINE FIELD IF NOT EXISTS image ON notes TYPE option<string>;
DEFINE FIELD IF NOT EXISTS created_at ON notes VALUE type::datetime($value);
DEFINE FIELD IF NOT EXISTS updated_at ON notes VALUE type::datetime($value);

DEFINE FIELD IF NOT EXISTS user_id on notes TYPE Record<users>;
DEFINE FIELD IF NOT EXISTS propertys on notes TYPE array<Record<propertys>>;
