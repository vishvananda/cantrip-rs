// Surge of Salvation
// status: ready
// You and permanents you control gain hexproof until end of turn. Prevent all damage that black and/or red sources would deal to creatures you control this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_3_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && (q.chars(source).colors.has(q.words(asker.source).color(Color::Black)) || q.chars(source).colors.has(q.words(asker.source).color(Color::Red))) && matches!(*target, Target::Object(o) if (q.chars(o).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(o)))) { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_0_0 { g.add_player_effect(player, source, EffectKind::PlayerHexproof, Duration::EndOfTurn); } }
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Hexproof])) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Surge of Salvation", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "You and permanents you control gain hexproof until end of turn. Prevent all damage that black and/or red sources would deal to creatures you control this turn.", damage: "damage", of_it: None, applies: made_3_applies, then: None, text: "Prevent all damage that black and/or red sources would deal to creatures you control this turn." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
