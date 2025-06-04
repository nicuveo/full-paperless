use paper_plane::schema::api::correspondents;
use paper_plane::schema::model::PermissionClass;
use paper_plane::services::Correspondents;

use crate::utils::client;

#[test]
fn test_correspondents_admin_crud() {
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
        let mut item = client
            .correspondents()
            .create(
                &correspondents::create(name.clone())
                    .matches("foo".to_string())
                    .is_insensitive(false),
            )
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!(Some(false), item.is_insensitive);
        assert_eq!(Some("foo".to_string()), item.matches);
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

        // update
        item.is_insensitive = Some(true);
        let item_copy = client.correspondents().update(&item).await?.value;
        assert_eq!(item, item_copy);

        // patch
        client
            .correspondents()
            .patch(
                &mut item,
                &correspondents::patch().name("-47r871qkds".to_string()),
            )
            .await?;

        // read
        let item_copy = client.correspondents().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // delete
        let id = item.id;
        client.correspondents().destroy(item).await?;
        assert_eq!(
            0,
            client
                .correspondents()
                .list(&correspondents::list())
                .await?
                .value
                .count
        );
        assert!(client.correspondents().retrieve(id).await.is_err());

        Ok(())
    })
}

#[test]
fn test_correspondents_user_crud() {
    let all_perms = vec![
        PermissionClass::CorrespondentAdd,
        PermissionClass::CorrespondentChange,
        PermissionClass::CorrespondentView,
        PermissionClass::CorrespondentDelete,
    ];
    client::run_as_user(vec![], all_perms, async |client| {
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
        let mut item = client
            .correspondents()
            .create(
                &correspondents::create(name.clone())
                    .matches("foo".to_string())
                    .is_insensitive(false),
            )
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!(Some(false), item.is_insensitive);
        assert_eq!(Some("foo".to_string()), item.matches);
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

        // update
        item.is_insensitive = Some(true);
        let item_copy = client.correspondents().update(&item).await?.value;
        assert_eq!(item, item_copy);

        // patch
        client
            .correspondents()
            .patch(
                &mut item,
                &correspondents::patch().name("-47r871qkds".to_string()),
            )
            .await?;

        // read
        let item_copy = client.correspondents().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // delete
        let id = item.id;
        client.correspondents().destroy(item).await?;
        assert_eq!(
            0,
            client
                .correspondents()
                .list(&correspondents::list())
                .await?
                .value
                .count
        );
        assert!(client.correspondents().retrieve(id).await.is_err());

        Ok(())
    })
}

#[test]
fn test_correspondents_nobody_crud() {
    client::run_as_user(vec![], vec![], async |client| {
        assert!(
            client
                .correspondents()
                .list(&correspondents::list())
                .await
                .is_err()
        );
        assert!(
            client
                .correspondents()
                .create(&correspondents::create("rainbiwdahd".to_string()))
                .await
                .is_err()
        );
        Ok(())
    })
}
