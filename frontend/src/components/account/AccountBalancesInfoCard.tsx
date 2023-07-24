import { getDollarTextFromCents } from "@/UIUtils"
import { Card, Paper, Stack} from "@mantine/core"
import { TextAndSubtext } from "../shared/TextAndSubtext"

interface AccountBalancesInfoPaperProps {
    availableBalanceCents: number,
    balanceCents: number
}

export const AccountBalancesInfoCard = ({ availableBalanceCents, balanceCents }: AccountBalancesInfoPaperProps) => {
    return <Card withBorder shadow="sm" radius="lg" p="md" h="11rem">
        <Stack>
            <TextAndSubtext text={getDollarTextFromCents(availableBalanceCents)} subText="Available Balance" textSize="xl" subTextSize="md" />
            <TextAndSubtext text={getDollarTextFromCents(balanceCents)} subText="Balance" textSize="xl" subTextSize="md" />
        </Stack>
    </Card>
}