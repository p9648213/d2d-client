#[derive(clap::Parser, Clone, Debug)]
pub struct EnvConfig {
	#[clap(long, env)]
	pub pg_host: String,
	#[clap(long, env)]
	pub pg_port: u16,
	#[clap(long, env)]
	pub pg_dbname: String,
	#[clap(long, env)]
	pub pg_user: String,
	#[clap(long, env)]
	pub pg_password: String,
	#[clap(long, env)]
	pub port: String,
	#[clap(long, env)]
	pub allow_origin: String,
	#[clap(long, env)]
	pub redis_url: String,
	#[clap(long, env)]
	pub csrf_encrypt_key: String,
	#[clap(long, env)]
	pub session_encrypt_key: String,
	#[clap(long, env)]
	pub database_encrypt_key: String,
	#[clap(long, env)]
	pub google_client_id: String,
	#[clap(long, env)]
	pub google_client_secret: String,
}
