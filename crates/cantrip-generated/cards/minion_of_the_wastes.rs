// Minion of the Wastes
// status: planned
// requires: numeric expression Game
// Trample
// As this creature enters, pay any amount of life.
// Minion of the Wastes's power and toughness are each equal to the life paid as it entered.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let mut done_0: i64 = 0; for player in { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() } { done_0 += { let most = g.state().player(player).life.clamp(0, 255) as u32; let paid = cantrip_mechanics::choose::number(g, player, asker.source, 0, most, "Pay")?; if paid > 0 && g.may_pay(player, asker.source, asker.source, life(paid as u8), &PAY_QUESTION)? { i64::from(paid) } else { 0 } }; }
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((planned_value!("numeric expression Game", 0i64 => q.game_value(item, GameValue::LifePaid))).clamp(-32768, 32767) as i16, (planned_value!("numeric expression Game", 0i64 => q.game_value(item, GameValue::LifePaid))).clamp(-32768, 32767) as i16); chars.power = Some(p); chars.toughness = Some(t); }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Minion of the Wastes", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Minion]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtSet, cantrip_mechanics::statics::itself, ability_2_apply_0, "Minion of the Wastes's power and toughness are each equal to the life paid as it entered.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
