#[cfg(test)]
mod tests {
    use dict::Dict;

    #[test]
    fn basics() {
        let mut d = Dict::new();

        assert_eq!(d.has("potato".to_string()), false, "does not contain default data");

        d.set("potato".to_string(), "foo".to_string());

        assert_eq!(d.has("potato".to_string()), true, "contains newly inserted data");

        assert_eq!(d.get("potato".to_string()), Some(&"foo".to_string()), "can retreive data");

        d.set("potato".to_string(), "foo2".to_string());

        assert_eq!(d.get("potato".to_string()), Some(&"foo2".to_string()), "data is overwritten");

        d.delete("potato".to_string());

        assert_eq!(d.has("potato".to_string()), false, "data is removed");
    }

    #[test]
    fn trans_cancel() {
        let mut d = Dict::new();

        d.begin();

        d.set("forgetable".to_string(), "such forget".to_string());

        assert_eq!(d.has("forgetable".to_string()), true, "value is retrievable when transaction is in progress");

        d.cancel();

        assert_eq!(d.has("forgetable".to_string()), false, "value is forgotten when transaction is canceled");
    }

    #[test]
    fn trans_overwrite_cancel() {
        let mut d = Dict::new();

        d.set("keepable".to_string(), "remember me".to_string());

        d.begin();

        d.set("keepable".to_string(), "fake".to_string());

        assert_eq!(d.get("keepable".to_string()), Some(&"fake".to_string()), "can retreive transaction value");

        d.cancel();

        assert_eq!(d.get("keepable".to_string()), Some(&"remember me".to_string()), "original value is restored after cancel");
    }

    #[test]
    fn trans_overwrite_commit() {
        let mut d = Dict::new();

        d.set("keepable".to_string(), "overwrite me".to_string());

        d.begin();

        d.set("keepable".to_string(), "new value".to_string());

        assert_eq!(d.get("keepable".to_string()), Some(&"new value".to_string()), "can retreive transaction value");

        d.commit();

        assert_eq!(d.get("keepable".to_string()), Some(&"new value".to_string()), "original value overwritten after commit");
    }

    #[test]
    fn trans_delete_cancel() {
        let mut d = Dict::new();

        d.set("keepable".to_string(), "remember me".to_string());

        d.begin();

        d.delete("keepable".to_string());

        assert_eq!(d.has("keepable".to_string()), false, "cannot retrieve delete entry mid transaction");

        d.cancel();

        assert_eq!(d.has("keepable".to_string()), true, "original value restored after cancel");
    }

    #[test]
    fn trans_delete_commit() {
        let mut d = Dict::new();

        d.set("forgetable".to_string(), "forget me".to_string());

        d.begin();

        d.delete("forgetable".to_string());

        assert_eq!(d.has("forgetable".to_string()), false, "cannot retrieve delete entry mid transaction");

        d.commit();

        assert_eq!(d.has("forgetable".to_string()), false, "original value deleted after commit");
    }
}
