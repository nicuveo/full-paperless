use paper_plane::clients::Client;
use paper_plane::schema::api::correspondents;
use paper_plane::services::Correspondents;

use crate::utils::client;

#[test]
fn correspondents_basic_crud() {
    client::run_as_admin(async |client| {
        // create
        assert_eq!(
            0,
            client
                .correspondents()
                .list(&correspondents::list())
                .await?
                .value
                .count
        );
        let name = "ffjfak'dlfa#f['pw/qnf.".to_string();
        let item = client
            .correspondents()
            .create(
                &correspondents::create(name.clone())
                    .matches("foo".to_string())
                    .is_insensitive(false),
            )
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!(false, item.is_insensitive);
        assert_eq!("foo".to_string(), item.matches);
        assert_eq!(
            1,
            client
                .correspondents()
                .list(&correspondents::list())
                .await?
                .value
                .count
        );

        // read
        let item_copy = client.correspondents().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // patch
        let name = "-47r871qkds".to_string();
        let item = client
            .correspondents()
            .patch(item.id, &correspondents::patch().name(name.clone()))
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!("foo".to_string(), item.matches);
        assert_eq!(
            1,
            client
                .correspondents()
                .list(&correspondents::list())
                .await?
                .value
                .count
        );

        // read
        let item_copy = client.correspondents().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // delete
        client.correspondents().destroy(item.id).await?;
        assert_eq!(
            0,
            client
                .correspondents()
                .list(&correspondents::list())
                .await?
                .value
                .count
        );
        assert!(client.correspondents().retrieve(item.id).await.is_err());

        Ok(())
    })
}

#[test]
fn correspondents_init_fields() {
    client::run_as_admin(async |client| {
        let item1 = client
            .correspondents()
            .create(&correspondents::create("tiwlgispqark".to_string()))
            .await?
            .value;
        let item2 = client.correspondents().retrieve(item1.id).await?.value;
        assert_eq!(item1, item2);
        Ok(())
    })
}
