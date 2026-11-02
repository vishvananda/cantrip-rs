// Bubbling Muck
// status: planned
// requires: mana for that player the source gives to you
// Until end of turn, whenever a player taps a Swamp for mana, that player adds an additional {B}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TappedForMana { permanent, player, produced } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), Mana::ALL.iter().filter(|&&m| produced.get(m) > 0).fold(0u32, |types, &m| types | 1 << m as u32)]; if true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Swamp)) { return Some(Fired { subject, data }); } }
 None }
fn watch_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("mana-for-that-player-the-source-gives-to-you", AddMana, "mana" = "{\"Amount\":{\"amount\":{\"args\":1,\"kind\":\"Constant\"},\"color\":\"Black\"}}", "players" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "restriction" = "()");
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Bubbling Muck", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), watch_1_event), watch_1, "Until end of turn, whenever a player taps a Swamp for mana, that player adds an additional {B}.").targets(&[]).mana_ability(TriggeredMana::One(Mana::B))], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
