import dotenv from "dotenv";
dotenv.config();

export const config = {
  rustApiUrl: process.env.RUST_API_URL || "http://localhost:8080",
  couchdbLocalUrl:
    process.env.COUCHDB_LOCAL_URL || "http://admin:password@localhost:5984",
  couchdbRemoteUrl:
    process.env.COUCHDB_REMOTE_URL || "http://admin:password@localhost:5984",
  dbName: process.env.COUCHDB_DB_NAME || "marketplace_pymes",
};
