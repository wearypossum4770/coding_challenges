pub fn remove_spaces(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    for c in input.chars() {
        if c != ' ' {
            buf.push(c);
        }
    }
    buf
}
#[cfg(test)]
mod tests {
    use super::remove_spaces;
    #[test]
    fn test_remove_spaces() {
        assert_eq!(
            remove_spaces("There is no wind in the football"),
            "Thereisnowindinthefootball"
        );
        assert_eq!(
            remove_spaces("I talk, he talk, why you middle talk?."),
            "Italk,hetalk,whyyoumiddletalk?."
        );
        assert_eq!(
            remove_spaces("You rotate the ground 4 times"),
            "Yourotatetheground4times"
        );
        assert_eq!(
            remove_spaces("You go and understand the tree."),
            "Yougoandunderstandthetree."
        );
        assert_eq!(
            remove_spaces("I'll give you clap on your cheeks"),
            "I'llgiveyouclaponyourcheeks"
        );
        assert_eq!(
            remove_spaces("Bring your parents and your mother and especially your father."),
            "Bringyourparentsandyourmotherandespeciallyyourfather."
        );
        assert_eq!(
            remove_spaces("Close the window airforce is coming."),
            "Closethewindowairforceiscoming."
        );
        assert_eq!(
            remove_spaces("I have two daughters and both are girls"),
            "Ihavetwodaughtersandbotharegirls"
        );
        assert_eq!(
            remove_spaces(" Stand in a straight circle"),
            "Standinastraightcircle"
        );
        assert_eq!(
            remove_spaces("Don't stand in front of my back"),
            "Don'tstandinfrontofmyback"
        );
        assert_eq!(remove_spaces("Why Haircut not cut?"), "WhyHaircutnotcut?");
        assert_eq!(
            remove_spaces("Don't make noise principle is rotating in the corridor"),
            "Don'tmakenoiseprincipleisrotatinginthecorridor"
        );
        assert_eq!(
            remove_spaces("Why are you looking at the monkey outside the window when I m here?"),
            "WhyareyoulookingatthemonkeyoutsidethewindowwhenImhere?"
        );
        assert_eq!(remove_spaces("You talking bad habit"), "Youtalkingbadhabit");
        assert_eq!(
            remove_spaces("Give me a red pen of any colour."),
            "Givemearedpenofanycolour."
        );
        assert_eq!(
            remove_spaces("Can I have some snow in my cold drink?"),
            "CanIhavesomesnowinmycolddrink?"
        );
        assert_eq!(
            remove_spaces("Pick the paper and fall into the dustbin."),
            "Pickthepaperandfallintothedustbin."
        );
        assert_eq!(
            remove_spaces("Both of you stand together separately."),
            "Bothofyoustandtogetherseparately."
        );
        assert_eq!(
            remove_spaces("Keep quiet the principal just passed away!."),
            "Keepquiettheprincipaljustpassedaway!."
        );
    }
}
