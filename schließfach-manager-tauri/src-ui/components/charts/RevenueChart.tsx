import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';

interface RevenueChartProps {
  data: Array<{ month: string; revenue: number }>;
}

export function RevenueChart({ data }: RevenueChartProps) {
  if (!data || data.length === 0) {
    return (
      <div className="h-[300px] flex items-center justify-center text-subtext-0">
        Keine Daten verfügbar
      </div>
    );
  }

  return (
    <div className="h-[300px] w-full">
      <ResponsiveContainer width="100%" height="100%">
        <BarChart
          data={data}
          margin={{ top: 5, right: 20, left: 0, bottom: 5 }}
        >
          <CartesianGrid
            strokeDasharray="3 3"
            stroke="var(--color-surface-1)"
            vertical={false}
          />
          <XAxis
            dataKey="month"
            tick={{ fill: 'var(--color-subtext-0)', fontSize: 12 }}
            tickLine={{ stroke: 'var(--color-surface-1)' }}
            axisLine={{ stroke: 'var(--color-surface-1)' }}
          />
          <YAxis
            tick={{ fill: 'var(--color-subtext-0)', fontSize: 12 }}
            tickLine={{ stroke: 'var(--color-surface-1)' }}
            axisLine={{ stroke: 'var(--color-surface-1)' }}
            tickFormatter={(value) => `${(value / 100).toFixed(0)}€`}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'var(--color-surface-0)',
              border: '1px solid var(--color-surface-1)',
              borderRadius: 'var(--radius-md)',
              color: 'var(--color-text)',
            }}
            formatter={(value: number) => [`${(value / 100).toFixed(2)}€`, 'Umsatz']}
            labelStyle={{ color: 'var(--color-subtext-0)' }}
          />
          <Bar
            dataKey="revenue"
            fill="var(--color-success)"
            radius={[4, 4, 0, 0]}
          />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
