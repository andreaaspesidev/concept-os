import plotly.express as px
import plotly.graph_objects as go
import pandas as pd

# ConceptOS (Raw measurements)
#           SYSCALL_0	SYSCALL_1	SYSCALL_2	SYSCALL_3	SYSCALL_4	SYSCALL_5	SYSCALL_6	SYSCALL_7	SYSCALL_9
# MAX	    21.792 μs	36.333 μs	17.042 μs	2.917 μs	17.833 μs	19.333 μs	9.417 μs	6.625 μs	2.958 μs
# AVG	    20.981 μs	31.351 μs	14.182 μs	2.050 μs	17.642 μs	18.015 μs	9.242 μs	6.366 μs	2.309 μs
# MIN	    4.875 μs	3.375 μs	14.083 μs	2.000 μs	17.458 μs	17.083 μs	9.208 μs	6.208 μs	1.667 μs
# AVG_CONF	[20.981 μs, [31.351 μs, [14.182 μs, [2.050 μs, [17.641 μs, [18.015 μs, [9.242 μs,  [6.366 μs,  [2.309 μs,
#            20.981 μs]  31.351 μs]	 14.182 μs]	 2.050 μs]	17.642 μs]	18.015 μs]	9.242 μs]	6.366 μs]	2.309 μs]


#            Hubris(max), ConceptOS(max)
max_data = [
    ["SEND (0)", 27.583, 21.000], # us
    ["RECV (1)", 26.167, 24.542],
    ["REPLY (2)", 18.875, 16.500],
    ["SET_TIMER (3)", 1.625, 2.125],
    ["BORROW_READ (4)", 20.875, 18.208],
    ["BORROW_WRITE (5)", 20.917, 18.250],
    ["BORROW_INFO (6)", 9.333, 9.375],
    ["IRQ_CONTROL (7)", 3.500, 6.458],
    ["GET_TIMER (9)", 1.958, 3.000],
]
#    Hubris(avg_low), cOS(avg_low), Hubris(avg_high), cOS(avg_high)
avg_data = [
    ["SEND (0)", 27.461, 15.378, 27.468, 15.844], # us
    ["RECV (1)", 13.419, 15.531, 13.781, 15.858],
    ["REPLY (2)", 18.760, 16.473, 18.762, 16.474],
    ["SET_TIMER (3)", 1.622, 2.096, 1.623, 2.098],
    ["BORROW_READ (4)", 20.825, 18.117, 20.826, 18.122],
    ["BORROW_WRITE (5)", 20.891, 18.143, 20.892, 18.145],
    ["BORROW_INFO (6)", 9.284, 9.299, 9.285, 9.304],
    ["IRQ_CONTROL (7)", 3.139, 6.338, 3.150, 6.341],
    ["GET_TIMER (9)", 1.531, 2.855,1.554, 2.865],
]

max_columns = ['Syscall', 'Hubris', 'ConceptOS']
df_max = pd.DataFrame(max_data, columns = max_columns)
df_max = df_max.set_index(max_columns[0])


fig = px.bar(df_max, x=max_columns[1:], text_auto=True, orientation='h', width=600, height=400, barmode='group')

fig.update_traces(
    textposition='outside',
    texttemplate='%{x:.3f}',
    textangle=0,
)
fig.update_layout(
    title="",
    legend_title_text="Operating System",
    yaxis_title="",
    xaxis_title="Time [us]<br>(sensitivity: ± 41.7ns)",
    yaxis=dict(autorange="reversed"),
    xaxis=dict(automargin=True, range=[0, 35], title_standoff = 0),
    margin=dict(
        b=10,
        l=10,
        r=10,
        t=10,
        pad = 5
    ),
    legend=dict(
      orientation="v",
      yanchor="bottom",
      y=0.02,
      xanchor="right",
      x=0.98
    ),
)
fig.write_image("build/syscall-timings-max.svg",width=600, height=400,scale=4)
fig.write_image("build/syscall-timings-max.png",width=600, height=400,scale=4)

# Elaborate avg
df_avg = pd.DataFrame(avg_data)
df_avg[5] = df_avg.apply(lambda x: (x[1] + x[2]) / 2, axis=1)
df_avg[6] = df_avg.apply(lambda x: x[5] - x[1], axis=1)
df_avg[7] = df_avg.apply(lambda x: x[2] - x[5], axis=1)
df_avg[8] = df_avg.apply(lambda x: (x[3] + x[4]) / 2, axis=1)
df_avg[9] = df_avg.apply(lambda x: x[8] - x[3], axis=1)
df_avg[10] = df_avg.apply(lambda x: x[4] - x[8], axis=1)

fig = go.Figure()

fig.add_trace(go.Bar(
      name="Hubris",
      y=df_avg[0], 
      x=df_avg[5],
      error_x=dict(type='data', array=df_avg[6]),
      orientation="h",
))
fig.add_trace(go.Bar(
      name="ConceptOS",
      y=df_avg[0], 
      x=df_avg[8],
      error_x=dict(type='data', array=df_avg[9]),
      orientation="h",
))

#fig.update_traces(
#    textposition='outside',
#    texttemplate='%{x:.3f}',
#    textangle=0,
#)
fig.update_layout(
    width=600, 
    height=400,
    title="",
    legend_title_text="Operating System",
    yaxis_title="",
    xaxis_title="Time [us]<br>(sensitivity: ± 41.7ns)",
    yaxis=dict(autorange="reversed"),
    xaxis=dict(automargin=True, range=[0, 30], title_standoff = 0),
    margin=dict(
        b=10,
        l=10,
        r=10,
        t=10,
        pad = 5
    ),
    legend=dict(
      orientation="v",
      yanchor="bottom",
      y=0.02,
      xanchor="right",
      x=0.98
    ),
)

fig.write_image("build/syscall-timings-avg.svg",width=600, height=400,scale=4)
fig.write_image("build/syscall-timings-avg.png",width=600, height=400,scale=4)