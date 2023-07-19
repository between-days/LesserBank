import { getAccountNumberString, getDollarText } from "@/UIUtils";
import { TransactionStatus } from "@/interfaces";
import { Card, Flex, Text, ThemeIcon } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { IconArrowBigRightLines, } from "@tabler/icons-react";

interface TransactionCardProps {
    fromNumber: number
    fromBsb: number
    toNumber: number
    toBsb: number
    fromName: string | undefined
    toName: string | undefined
    amountCents: number
    availableBalanceCents: number
    status: TransactionStatus
    date: Date
}

export default function TransactionCard({ fromNumber, fromBsb, toNumber, toBsb, fromName, toName, amountCents, availableBalanceCents, status, date, }: TransactionCardProps) {
    const { hovered, ref } = useHover();

    return (
        <Card ref={ref} shadow={hovered ? "xl" : "sm"} withBorder={hovered ? false : true} radius="md" padding="md">
            <Flex
                mih={50}
                gap="md"
                justify="space-between"
                wrap="wrap"
            >
                <Flex
                    mih={50}
                    gap="md"
                    justify="flex-end"
                    align="flex-start"
                    direction="row"
                    wrap="wrap"
                >
                    <Flex
                        mih={50}

                        justify="flex-start"
                        align="flex-start"
                        direction="column"
                        wrap="wrap"
                    >
                        <Text fz="xl" weight={500}>
                            {date.toLocaleDateString()}
                        </Text>
                        <Text color="green"  >
                            {getDollarText(amountCents)}
                        </Text>
                    </Flex>
                </Flex>
                <Flex
                    mih={50}
                    gap="lg"
                    justify="center"
                    align="center"
                    direction="column"
                    wrap="wrap"
                >
                    <Flex
                        mih={50}
                        gap="md"
                        justify="flex-start"
                        align="center"
                        direction="row"
                        wrap="wrap"
                    >
                        <div >
                            <Text fw={500}>{fromName}</Text>
                            <Text color="dimmed">{getAccountNumberString(fromNumber)}</Text>
                        </div>

                        <ThemeIcon>
                            <IconArrowBigRightLines />
                        </ThemeIcon>

                        <div>
                            <Text fw={500}>{toName}</Text>
                            <Text color="dimmed">{getAccountNumberString(toNumber)}</Text>
                        </div>
                    </Flex>
                    <Text fz="lg" fw={500} >Available: {getDollarText(availableBalanceCents)}</Text>
                </Flex>
            </Flex>
        </Card>
    );
}