// Avengers Assemble!
// status: ready
// Flash
// Heroes you control get +2/+2.
// At the beginning of each end step, if you attacked with a Hero this turn or a Hero entered the battlefield under your control this turn, draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Hero)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (2i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { (({ let mut attackers: Vec<ObjectId> = q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defending, known, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if ((Attacked { attacker: id, known: *known }).chars(q).has_subtype(q.words(asker.source).subtype(Subtype::Hero)) && Some(asker.controller) == Some(r.active)) { return Some(Fired { subject, data }); } }  None }).filter_map(|f: Fired| f.subject.object()).collect(); attackers.sort(); attackers.dedup(); attackers.len() as i64 }) >= (1i64) || (q.history().entered().filter(|e| (e.chars(q).has_subtype(q.words(asker.source).subtype(Subtype::Hero)) && Some(asker.controller) == Some(e.controller))).count() as i64) >= (1i64)) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Avengers Assemble!", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of each end step, if you attacked with a Hero this turn or a Hero entered the battlefield under your control this turn, draw a card.").targets(&[]).condition(ability_2_if)], statics: &[static_ability(Layer::PtModify, ability_1_affects, ability_1_apply_0, "Heroes you control get +2/+2.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
