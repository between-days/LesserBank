import { getAccountNumberString, getDollarText } from "@/UIUtils";
import { Card, Flex, Text, ThemeIcon } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { IconArrowBigRightLines, } from "@tabler/icons-react";

interface TransactionCardProps {
    fromNumber: number
    toNumber: number
    fromName: string
    toName: string
    amountCents: number
}

export default function TransactionCard({ fromNumber, toNumber, fromName, toName, amountCents }: TransactionCardProps) {
    const { hovered, ref } = useHover();

    return (
        <Card ref={ref} shadow={hovered ? "xl" : "sm"} withBorder={hovered ? false : true} radius="md" padding="md">
            <Flex
                mih={50}
                gap="md"
                justify="space-between"
                align="center"
                direction="row"
                wrap="wrap"
            >
                <Text color="green" weight={500} fz="xl">
                    {getDollarText(amountCents)}
                </Text>

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
            </Flex>
        </Card>
    );
}