// Raging River
// status: planned
// requires: a lasting block rule reading what its ability chose
// requires: a pile's name kept on its objects for a lasting rule
// requires: player collection Scoped
// Whenever one or more creatures you control attack, each defending player divides all creatures without flying they control into a "left" pile and a "right" pile. Then, for each attacking creature you control, choose "left" or "right." That creature can't be blocked this combat except by creatures with flying and creatures in a pile with the chosen label.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|p: PlayerId| (planned_value!("player collection Scoped", Vec::<PlayerId>::new() => q.players_in_scope(asker))).contains(&p))).collect::<Vec<PlayerId>>() };
let mut piles_0: Vec<(PlayerId, Vec<Vec<ObjectId>>, Option<usize>, Vec<PlayerId>)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("a-pile-s-name-kept-on-its-objects-for-a-lasting-rule" => cantrip_mechanics::choose::into_piles(g, { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !(q.chars(id).keywords.has(Keyword::Flying)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }, None, &["Left", "Right"], None)?);
    } 
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() } {
planned!("a-pile-s-name-kept-on-its-objects-for-a-lasting-rule" => cantrip_mechanics::choose::of_piles(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, None, false, &["Left", "Right"], { let q: &dyn Query = g; Some(bound_0) }, None)?);
g.end_sentence();
planned!("a-lasting-block-rule-reading-what-its-ability-chose", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":0,\"kind\":\"Bound\"},\"kind\":\"One\"}}", "changes" = "[{\"args\":{\"Prohibit\":{\"BeBlocked\":{\"args\":{\"args\":[{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"kind\":\"Flying\"},\"kind\":\"HasKeyword\"}],\"kind\":\"All\"},{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"args\":0,\"kind\":\"Bound\"},\"kind\":\"InPile\"}],\"kind\":\"All\"}],\"kind\":\"Either\"},\"kind\":\"Not\"}}},\"kind\":\"Rule\"}]", "duration" = "\"EndOfCombat\"", "evaluation" = "\"LiveObjects\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Raging River", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever one or more creatures you control attack, each defending player divides all creatures without flying they control into a \"left\" pile and a \"right\" pile. Then, for each attacking creature you control, choose \"left\" or \"right.\" That creature can't be blocked this combat except by creatures with flying and creatures in a pile with the chosen label.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
