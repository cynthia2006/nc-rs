use tokio::io::{self, stdin, stdout};
use tokio::net::{TcpListener, TcpStream};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Address and port to connect to
    uri: String,

    /// Listen instead of connecting
    #[arg(short, long)]
    listen: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let socket = if cli.listen {
        TcpListener::bind(cli.uri).await?.accept().await?.0
    } else {
        TcpStream::connect(cli.uri).await?
    };
    let (mut rd, mut wr) = socket.into_split();

    let writer = tokio::spawn(async move {
        let mut stdin = stdin();

        io::copy(&mut stdin, &mut wr).await?;
        Ok::<_, io::Error>(())
    });

    let reader = tokio::spawn(async move {
        let mut stdout = stdout();

        io::copy(&mut rd, &mut stdout).await?;
        Ok::<_, io::Error>(())
    });

    writer.await??;
    reader.await??;

    Ok(())
}
