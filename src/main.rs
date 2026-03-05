use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Usuario{
    login:String,
    name:Option<String>,
    public_repos:u32,
    followers:u32,
}

async fn buscar_usuario(username:&str)->Result<Usuario,reqwest::Error>{
    let url=format!("https://api.github.com/users/{}",username);
    let usuario=reqwest::Client::new()
        .get(&url)
        .header("User-Agent","buscador-rust")
        .send()
        .await?
        .json::<Usuario>()
        .await?;
    Ok(usuario)
}



#[tokio::main]
async fn main(){
    println!("=== Buscador de user no gitbub ===");

    println!("Digite o nome do usuário do github:");
    let name= std::io::stdin()
        .lines()
        .next()
        .expect("Erro ao ler a entrada")
        .expect("Erro ao ler a entrada");

    match buscar_usuario(&name).await {
        Ok(usuario) =>{
            println!("LOGIN {}",usuario.login);
            match usuario.name {
                Some(name)=>{
                    println!("Nome  {}",name);
                },
                None=>println!("Nome  não informado"),
            }
            println!("Followers: {}",usuario.followers);
            println!("Public Repos: {}",usuario.public_repos);
        }
        Err(e) =>{
            eprintln!("erro ao buscar o user {}",e)
        }


    }
}