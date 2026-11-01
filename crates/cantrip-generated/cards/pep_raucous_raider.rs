// Pep, Raucous Raider
// status: planned
// requires: engine: no perpetual effects
// Trample, Haste
// Whenever a creature you control deals combat damage to a player, exile the top card of that player's library. It perpetually becomes an artifact if it's a nonland permanent card. Until end of turn, you may play the exiled card.
// {T}, Sacrifice an artifact: Add three mana of any one color.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
if { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().is_some_and(|candidate: ObjectId| (!(q.chars(candidate).is(CardType::Land)) && (q.chars(candidate).is(CardType::Artifact) || q.chars(candidate).is(CardType::Battle) || q.chars(candidate).is(CardType::Creature) || q.chars(candidate).is(CardType::Enchantment) || q.chars(candidate).is(CardType::Land) || q.chars(candidate).is(CardType::Planeswalker)))) } {
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddTypes(TypeSet::of(&[CardType::Artifact]))) };
for &object in &objects_2 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
}
g.end_sentence();
let cards_3_0 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_3_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_3_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_3_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
g.end_sentence();
 Ok(()) }
fn ability_3_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_4 = { let q: &dyn Query = g; (3i64).max(0) as u32 };
for player in mana_players_4 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_4);}
 Ok(()) }
#[allow(unused_variables)] fn ability_3_5(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Artifact) }

pub static CARD: CardDef = CardDef { name: "Pep, Raucous Raider", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Noggle,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever a creature you control deals combat damage to a player, exile the top card of that player's library. It perpetually becomes an artifact if it's a nonland permanent card. Until end of turn, you may play the exiled card.").targets(&[]),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(sacrifice(1, ability_3_5)), add_by(ability_3_mana).choose_color().colors(ColorSet(31)).yields_any(3), "{T}, Sacrifice an artifact: Add three mana of any one color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
