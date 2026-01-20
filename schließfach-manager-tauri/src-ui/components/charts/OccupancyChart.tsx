import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';
import type { OccupancyHistoryPoint } from '@/types';

interface OccupancyChartProps {
  data: OccupancyHistoryPoint[];
}

export function OccupancyChart({ data }: OccupancyChartProps) {
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
        <LineChart
          data={data}
          margin={{ top: 5, right: 20, left: 0, bottom: 5 }}
        >
          <CartesianGrid
            strokeDasharray="3 3"
            stroke="var(--color-surface-1)"
          />
          <XAxis
            dataKey="date"
            tick={{ fill: 'var(--color-subtext-0)', fontSize: 12 }}
            tickLine={{ stroke: 'var(--color-surface-1)' }}
            axisLine={{ stroke: 'var(--color-surface-1)' }}
          />
          <YAxis
            domain={[0, 100]}
            tick={{ fill: 'var(--color-subtext-0)', fontSize: 12 }}
            tickLine={{ stroke: 'var(--color-surface-1)' }}
            axisLine={{ stroke: 'var(--color-surface-1)' }}
            tickFormatter={(value) => `${value}%`}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'var(--color-surface-0)',
              border: '1px solid var(--color-surface-1)',
              borderRadius: 'var(--radius-md)',
              color: 'var(--color-text)',
            }}
            formatter={(value: number) => [`${value.toFixed(1)}%`, 'Belegung']}
            labelStyle={{ color: 'var(--color-subtext-0)' }}
          />
          <Line
            type="monotone"
            dataKey="percent"
            stroke="var(--color-primary)"
            strokeWidth={2}
            dot={false}
            activeDot={{
              r: 4,
              fill: 'var(--color-primary)',
              stroke: 'var(--color-base)',
              strokeWidth: 2,
            }}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
