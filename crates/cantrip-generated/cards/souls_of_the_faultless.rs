// Souls of the Faultless
// status: ready
// Defender (This creature can't attack.)
// Whenever this creature is dealt combat damage, you gain that much life and attacking player loses that much life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = *target; let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if combat && true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.data[0])).min(i32::MAX.into()) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (i64::from(item.data[0])).min(i32::MAX.into()) as i32 }; let players_1 = { let q: &dyn Query = g; Some(q.state().turn.active).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Souls of the Faultless", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever this creature is dealt combat damage, you gain that much life and attacking player loses that much life.").targets(&[]).sums_per_subject()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
