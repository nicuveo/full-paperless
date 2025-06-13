use paper_plane::schema::api::mail_accounts;
use paper_plane::services::MailAccounts;

use crate::utils::client;

#[test]
fn test_mail_accounts_basic_crud() {
    client::run_as_admin(async |client| {
        // create
        assert_eq!(
            0,
            client
                .mail_accounts()
                .list(&mail_accounts::list())
                .await?
                .value
                .count
        );
        let imap = "foo@foo.com".to_string();
        let name = "ffjfak'dlfa#f['pw/qnf.".to_string();
        let username = "iu8y1r7ohj028twilidh92".to_string();
        let password = "hfo78yp98;ihddslabf0pi".to_string();
        let mut item = client
            .mail_accounts()
            .create(&mail_accounts::create(
                name.clone(),
                imap.clone(),
                username.clone(),
                password.clone(),
            ))
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!(imap, item.imap_server);
        assert_eq!(username, item.username);
        assert_eq!(password, item.password);
        assert_eq!(
            1,
            client
                .mail_accounts()
                .list(&mail_accounts::list())
                .await?
                .value
                .count
        );

        // read
        let item_copy = client.mail_accounts().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // update
        let name = "-47r871qkds".to_string();
        item.name = name.clone();
        let item_copy = client.mail_accounts().update(&item).await?.value;
        assert_eq!(item, item_copy);

        // patch
        let username = "vrcelesiawhlhqt".to_string();
        let password = "vrcelesiawhlhqt".to_string();
        let item = client
            .mail_accounts()
            .patch(
                item.id,
                &mail_accounts::patch()
                    .username(username.clone())
                    .password(password.clone()),
            )
            .await?
            .value;
        assert_eq!(name, item.name);
        assert_eq!(imap, item.imap_server);
        assert_eq!(username, item.username);
        assert_eq!(password, item.password);
        assert_eq!(
            1,
            client
                .mail_accounts()
                .list(&mail_accounts::list())
                .await?
                .value
                .count
        );

        // read
        let item_copy = client.mail_accounts().retrieve(item.id).await?.value;
        assert_eq!(item, item_copy);

        // delete
        client.mail_accounts().destroy(item.id).await?;
        assert_eq!(
            0,
            client
                .mail_accounts()
                .list(&mail_accounts::list())
                .await?
                .value
                .count
        );
        assert!(client.mail_accounts().retrieve(item.id).await.is_err());

        Ok(())
    })
}

#[test]
fn mail_accounts_init_accounts() {
    client::run_as_admin(async |client| {
        let item1 = client
            .mail_accounts()
            .create(&mail_accounts::create(
                "tiwlispqark".to_string(),
                "foo@foo.com".to_string(),
                "therairrirtty1".to_string(),
                "flfljtuteerhsy".to_string(),
            ))
            .await?
            .value;
        let item2 = client.mail_accounts().retrieve(item1.id).await?.value;
        assert_eq!(item1, item2);
        Ok(())
    })
}
