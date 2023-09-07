use mysql::*;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub fn run_migrations() {
    let url = "mysql://root:root@localhost:3306/api";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}
