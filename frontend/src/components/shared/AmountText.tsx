import { getDollarTextFromCents } from "@/UIUtils"
import { Text } from "@mantine/core"

export const AmountText = ({ amountCents }: { amountCents: number }) => {
    const dollarString = getDollarTextFromCents(amountCents)
    const parts = dollarString.split('.')

    switch (parts.length) {
        case 2:
            return <Text fw="bold" fz="lg">
                {parts[0]}<Text span fz="sm">.{parts[1]}</Text>
            </Text>
        default:
            return <Text fw="bold" fz="lg">
                {dollarString}
            </Text>
    }
}