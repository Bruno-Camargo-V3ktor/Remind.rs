DEFINE TABLE IF NOT EXISTS propertys SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS id ON propertys TYPE UUID;
DEFINE FIELD IF NOT EXISTS name ON propertys TYPE string;
DEFINE FIELD IF NOT EXISTS color ON propertys TYPE number;
DEFINE FIELD IF NOT EXISTS value ON propertys TYPE number;
DEFINE FIELD IF NOT EXISTS type ON propertys TYPE "Category" | "Priority";
DEFINE FIELD IF NOT EXISTS created_at ON TABLE propertys VALUE type::datetime($value);
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE propertys VALUE type::datetime($value);

DEFINE FIELD IF NOT EXISTS user_id on propertys TYPE Record<users>;
