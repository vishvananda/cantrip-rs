// Bismuth Mindrender
// status: planned
// requires: operation PlayWith
// Devoid (This card has no color.)
// Menace
// Whenever this creature deals combat damage to a player, that player exiles cards from the top of their library until they exile a nonland card. You may cast that card by paying life equal to the spell's mana value rather than paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.colors = ColorSet::NONE; }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let owner_0 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_0, rest_0) = cantrip_mechanics::library::reveal_until_n(g, owner_0, count_0, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_0 = found_0.len() + rest_0.len(); let objects_0: Vec<ObjectId> = g.state().zone(Zone::library(owner_0)).iter().copied().take(shown_0).collect();
let objects_1 = { let q: &dyn Query = g; objects_0.clone() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? {
planned!("operation-play-with" => cantrip_mechanics::play::with_terms(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, CardPlay::CastSpell, { let q: &dyn Query = g; ((moved_1.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }, { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }, None, "{\"limit\":null,\"during\":null,\"cost\":{\"Alternate\":{\"kind\":\"Cost\",\"args\":{\"parts\":[\"LifeEqualToOwnManaValue\"]}}},\"spend_as\":null,\"into_exile\":false,\"condition\":null,\"effects\":[],\"flash\":false,\"may_look\":false,\"then\":null,\"face_down\":null}")?);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Bismuth Mindrender", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever this creature deals combat damage to a player, that player exiles cards from the top of their library until they exile a nonland card. You may cast that card by paying life equal to the spell's mana value rather than paying its mana cost.").targets(&[])], statics: &[characteristic_defining(Layer::Color, ability_0_apply_0, "Devoid")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
