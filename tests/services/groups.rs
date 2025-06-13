use paper_plane::schema::api::groups;
use paper_plane::schema::model::PermissionClass;
use paper_plane::services::Groups;

use crate::utils::client;

#[test]
fn groups_basic_crud() {
    client::run_as_admin(async |client| {
        // create
        assert_eq!(0, client.groups().list(&groups::list()).await?.value.count);
        let name = "ffjfak'dlfa#f['pw/qnf.".to_string();
        let mut perms = vec![PermissionClass::UserDelete, PermissionClass::DocumentChange];
        let mut item = client
            .groups()
            .create(&groups::create(name.clone(), perms.clone()))
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!(perms, item.permissions);
        assert_eq!(1, client.groups().list(&groups::list()).await?.value.count);

        // read
        let item_copy = client.groups().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // update
        perms.push(PermissionClass::NoteAdd);
        item.permissions = perms.clone();
        let item_copy = client.groups().update(&item).await?.value;
        assert_eq!(item, item_copy);

        // patch
        let name = "-47r871qkds".to_string();
        let item = client
            .groups()
            .patch(item.id, &groups::patch().name(name.clone()))
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!(perms, item.permissions);
        assert_eq!(1, client.groups().list(&groups::list()).await?.value.count);

        // read
        let item_copy = client.groups().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // delete
        client.groups().destroy(item.id).await?;
        assert_eq!(0, client.groups().list(&groups::list()).await?.value.count);
        assert!(client.groups().retrieve(item.id).await.is_err());

        Ok(())
    })
}

#[test]
fn groups_init_fields() {
    client::run_as_admin(async |client| {
        let item1 = client
            .groups()
            .create(&groups::create("tiwlgispqark".to_string(), vec![]))
            .await?
            .value;
        let item2 = client.groups().retrieve(item1.id).await?.value;
        assert_eq!(item1, item2);
        Ok(())
    })
}
