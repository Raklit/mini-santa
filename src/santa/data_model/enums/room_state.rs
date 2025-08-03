use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RoomState {
    ChosingAGift = 0,
    BuyingAGift = 1,
    MailerAwaitingGiftDelivery = 2,
    GiftDeliveredToMailer = 3,
    MailerSendGiftToRecipient = 4,
    GiftInAWayToRecipient = 5,
    GiftHasBeenDeliveredToRecipient = 6,
    RecipientTookTheGift = 7
}

impl TryFrom<usize> for RoomState {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            value if value == RoomState::ChosingAGift as usize => Ok(RoomState::ChosingAGift),
            value if value == RoomState::BuyingAGift as usize => Ok(RoomState::BuyingAGift),
            value if value == RoomState::MailerAwaitingGiftDelivery as usize => Ok(RoomState::MailerAwaitingGiftDelivery),
            value if value == RoomState::GiftDeliveredToMailer as usize => Ok(RoomState::GiftDeliveredToMailer),
            value if value == RoomState::MailerSendGiftToRecipient as usize => Ok(RoomState::MailerSendGiftToRecipient),
            value if value == RoomState::GiftInAWayToRecipient as usize => Ok(RoomState::GiftInAWayToRecipient),
            value if value == RoomState::GiftHasBeenDeliveredToRecipient as usize => Ok(RoomState::GiftHasBeenDeliveredToRecipient),
            value if value == RoomState::RecipientTookTheGift as usize => Ok(Self::RecipientTookTheGift),
            _ => Err(())
        }
    }
}