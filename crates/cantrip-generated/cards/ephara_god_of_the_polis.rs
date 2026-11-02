// Ephara, God of the Polis
// status: planned
// requires: numeric expression CountManaSymbols
// Indestructible
// As long as your devotion to white and blue is less than seven, Ephara isn't a creature.
// At the beginning of each upkeep, if you had another creature enter the battlefield under your control last turn, draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((planned_value!("numeric expression CountManaSymbols", 0i64 => i64::from(cantrip_mechanics::devotion::symbols_among(q, &q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>(), &[Color::White.mana(), Color::Blue.mana()])))) < (7i64)) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.types = TypeSet(chars.types.0 & !(1 << CardType::Creature as u8)); }
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { { let of = Some(asker.controller); let turn = q.history().all().iter().rev().find(|r| Some(r.active) == of && r.turn < q.state().turn.number).map(|r| r.turn); q.history().all().iter().filter(move |r| Some(r.turn) == turn) }.filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::ZoneChange { new, to, .. } = event { let id = *new; let player = q.controller(id); let subject = Target::Object(id); let data = [new.0, u32::from(player.0)]; if to.is(ZoneKind::Battlefield) && Some(asker.controller) == Some(player) && (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }  None }).next().is_some() }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ephara, God of the Polis", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of each upkeep, if you had another creature enter the battlefield under your control last turn, draw a card.").targets(&[]).condition(ability_2_if)], statics: &[static_ability(Layer::Type, ability_1_0_affects, ability_1_0_apply_0, "As long as your devotion to white and blue is less than seven, Ephara isn't a creature.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
