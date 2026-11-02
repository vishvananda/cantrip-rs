// Aetherblade Agent // Gitaxian Mindstinger
// status: ready
// Deathtouch
// Whenever this creature deals combat damage to a player or battle, draw a card.
// Deathtouch
// {4}{U/P}: Transform this creature. Activate only as a sorcery. ({U/P} can be paid with either {U} or 2 life.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Aetherblade Agent", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 1, 0, 0, 0] })), ability_1, "{4}{U/P}: Transform this creature. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Object(o) if q.chars(o).is(CardType::Battle)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gitaxian Mindstinger", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(6)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever this creature deals combat damage to a player or battle, draw a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
