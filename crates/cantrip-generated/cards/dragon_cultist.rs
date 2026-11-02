// Dragon Cultist
// status: planned
// requires: engine: no commander
// Commander creatures you own have "At the beginning of your end step, if a source you controlled dealt 5 or more damage this turn, create a 4/4 red Dragon creature token with flying."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (planned_value!("engine: no commander", false => q.is_commander(affected)) && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.state().obj(affected).owner)) }
fn granted_0_if(q: &dyn Query, asker: &Asker) -> bool { ({ let mut most: Vec<(Target, u32, i64)> = Vec::new(); for f in q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Damage { source, target, amount, kind, controller, known, .. } = event { let (source, amount, combat, controller) = (*source, *amount, kind.is_combat(), *controller); let dealt = DamageDealt { source, target: *target, amount, combat, controller, known: *known }; let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Object(o) => o.0, Target::Player(p) => u32::MAX - 1 - u32::from(p.0), _ => u32::MAX }]; if true && Some(asker.controller) == Some(controller) && true { return Some(Fired { subject, data }); } }  None }) { let key: u32 = 0; match most.iter_mut().find(|d| d.0 == f.subject && d.1 == key) { Some(d) => d.2 += i64::from(f.data[0]), None => most.push((f.subject, key, i64::from(f.data[0]))) } } most.into_iter().map(|d| d.2).max().unwrap_or(0) }) >= (5i64) }
#[allow(unused_variables)] fn granted_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Dragon).name, token_words: token!(Dragon).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(8)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn granted_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }

pub static CARD: CardDef = CardDef { name: "Dragon Cultist", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Background]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_0, "Commander creatures you own have \"At the beginning of your end step, if a source you controlled dealt 5 or more damage this turn, create a 4/4 red Dragon creature token with flying.\"")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_0_event), granted_0, "At the beginning of your end step, if a source you controlled dealt 5 or more damage this turn, create a 4/4 red Dragon creature token with flying.").targets(&[]).condition(granted_0_if)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
