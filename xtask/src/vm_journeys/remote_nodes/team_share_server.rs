use std::{
    fs, io,
    net::{IpAddr, SocketAddr},
    os::unix::fs::{PermissionsExt, chown},
    path::PathBuf,
    sync::Arc,
};

use axum::{Router, extract::State, http::StatusCode, routing::put};

const COLLEAGUE_USER_ID: u32 = 1000;
const COLLEAGUE_GROUP_ID: u32 = 100;
const PLACED_SAMPLE_MODE: u32 = 0o640;

#[derive(clap::Args, Debug)]
pub struct TeamSharePlacementOptions {
    #[arg(long)]
    sample_directory: PathBuf,
    #[arg(long)]
    share_directory: PathBuf,
    #[arg(long)]
    listen_address: IpAddr,
    #[arg(long)]
    port: u16,
}

struct TeamShareDirectories {
    sample_directory: PathBuf,
    share_directory: PathBuf,
}

fn place_sample_file(
    share_directories: &TeamShareDirectories,
    sample_name: &str,
) -> io::Result<PathBuf> {
    let placed_path = share_directories.share_directory.join(sample_name);
    fs::copy(
        share_directories.sample_directory.join(sample_name),
        &placed_path,
    )?;
    chown(
        &placed_path,
        Some(COLLEAGUE_USER_ID),
        Some(COLLEAGUE_GROUP_ID),
    )?;
    fs::set_permissions(&placed_path, fs::Permissions::from_mode(PLACED_SAMPLE_MODE))?;
    Ok(placed_path)
}

async fn place_sample(
    State(share_directories): State<Arc<TeamShareDirectories>>,
    sample_name: String,
) -> (StatusCode, String) {
    if sample_name.is_empty() || sample_name.contains('/') {
        return (
            StatusCode::BAD_REQUEST,
            format!("{sample_name:?} is not a sample name\n"),
        );
    }
    match place_sample_file(&share_directories, &sample_name) {
        Ok(placed_path) => (StatusCode::CREATED, format!("{}\n", placed_path.display())),
        Err(placement_failure) => (
            StatusCode::NOT_FOUND,
            format!("{sample_name:?} could not be placed: {placement_failure}\n"),
        ),
    }
}

pub fn serve_team_share_placement(placement_options: TeamSharePlacementOptions) -> io::Result<()> {
    let placement_router = Router::new()
        .route("/place", put(place_sample))
        .with_state(Arc::new(TeamShareDirectories {
            sample_directory: placement_options.sample_directory,
            share_directory: placement_options.share_directory,
        }));
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let placement_listener = tokio::net::TcpListener::bind(SocketAddr::new(
                placement_options.listen_address,
                placement_options.port,
            ))
            .await?;
            axum::serve(placement_listener, placement_router).await
        })
}
