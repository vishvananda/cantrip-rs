// Ornate Imitations
// status: planned
// requires: numeric expression AnnouncedX
// requires: numeric expression Game
// requires: operation EachNumber
// For each number between 1 and X, conjure a random creature card with that mana value onto the battlefield. X can't be 0.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (planned_value!("numeric expression AnnouncedX", 0i64 => i64::from(q.announced_x().unwrap_or(0)))) != (0i64) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-each-number" => for number in { let q: &dyn Query = g; 1i64 }..={ let q: &dyn Query = g; i64::from(item.x) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::AtRandom(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) == (planned_value!("numeric expression Game", 0i64 => q.game_value(item, GameValue::EachValue))))), { let q: &dyn Query = g; 1i64 }, ConjureTo::Entering(EventZone::Battlefield, ConjureSpot::Random, &[EnterFlag::EntersNormally]))?; }; });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ornate Imitations", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_only_if: Some(ability_0_if), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
