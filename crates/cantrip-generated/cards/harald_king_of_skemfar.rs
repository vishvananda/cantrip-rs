// Harald, King of Skemfar
// status: ready
// Menace (This creature can't be blocked except by two or more creatures.)
// When Harald enters, look at the top five cards of your library. You may reveal an Elf, Warrior, or Tyvar card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_0 = { let q: &dyn Query = g; (5i64).max(0) as usize }; let word_0_0 = { let q: &dyn Query = g; q.words(asker.source).subtype(Subtype::Elf) }; let word_0_1 = { let q: &dyn Query = g; q.words(asker.source).subtype(Subtype::Warrior) }; let word_0_2 = { let q: &dyn Query = g; q.words(asker.source).subtype(Subtype::Tyvar) }; let taken_0 = cantrip_mechanics::library::look_at_top(g, player_0, asker.source, depth_0, (0, 1), true, |c| (c.has_subtype(word_0_0) || c.has_subtype(word_0_1) || c.has_subtype(word_0_2)), cantrip_mechanics::library::Rest::BottomRandom)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Harald, King of Skemfar", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When Harald enters, look at the top five cards of your library. You may reveal an Elf, Warrior, or Tyvar card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
