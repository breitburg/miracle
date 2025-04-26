import 'package:flutter/cupertino.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:gap/gap.dart';
import 'package:langchain/langchain.dart';
import 'package:langchain_openai/langchain_openai.dart';
import 'package:macos_ui/macos_ui.dart';

class HomeScreen extends StatefulWidget {
  static const String routeName = '/';

  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final chat = ChatOpenAI(
    baseUrl: 'http://localhost:11434/v1',
    apiKey: 'ollama',
    defaultOptions: ChatOpenAIOptions(model: 'gemma3:4b-it-qat'),
  );

  final history = <ChatMessage>[];
  final _controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return MacosWindow(
      sidebar: Sidebar(
        top: MacosSearchField(
          placeholder: 'Search',
          onResultSelected: (result) {},
        ),

        builder: (context, scrollController) {
          return SidebarItems(
            currentIndex: 1,
            scrollController: scrollController,
            onChanged: (i) {},
            items: const [
              SidebarItem(label: Text('Projects'), section: true),
              SidebarItem(
                label: Row(
                  children: [
                    Icon(
                      CupertinoIcons.add,
                      size: 16,
                      color: MacosColors.systemGrayColor,
                    ),
                    Gap(10),
                    Text(
                      'Create new project',
                      style: TextStyle(color: MacosColors.systemGrayColor),
                    ),
                  ],
                ),
              ),
              SidebarItem(label: Text('Today'), section: true),
              SidebarItem(label: Text('Binary programming')),
              SidebarItem(label: Text('Why is the sky blue')),
              SidebarItem(label: Text('Yesterday'), section: true),
              SidebarItem(
                label: Text('The difference between a cat and a dog'),
              ),
              SidebarItem(label: Text('Theory of relativity explained')),
              SidebarItem(label: Text('3 days ago'), section: true),
              SidebarItem(label: Text('How to grow tomatoes indoors')),
              SidebarItem(label: Text('Python vs. JavaScript')),
              SidebarItem(label: Text('2 weeks ago'), section: true),
              SidebarItem(label: Text('Recipe for banana bread')),
              SidebarItem(label: Text('Climate change effects')),
              SidebarItem(label: Text('How to deploy a React website')),
              SidebarItem(label: Text('1 month ago'), section: true),
              SidebarItem(label: Text('macOS shortcuts guide')),
              SidebarItem(label: Text('How to learn Flutter')),
              SidebarItem(label: Text('Deep learning explained')),
            ],
          );
        },
        minWidth: 200,
      ),
      child: Builder(
        builder: (context) {
          return MacosScaffold(
            toolBar: ToolBar(
              padding: const EdgeInsets.only(left: 15, right: 10),
              leading: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  MacosIconButton(
                    icon: MacosIcon(
                      CupertinoIcons.sidebar_left,
                      color: MacosTheme.brightnessOf(context).resolve(
                        const Color.fromRGBO(0, 0, 0, 0.5),
                        const Color.fromRGBO(255, 255, 255, 0.5),
                      ),
                      size: 20.0,
                    ),
                    boxConstraints: BoxConstraints(),
                    onPressed: () {
                      MacosWindowScope.of(context).toggleSidebar();
                    },
                  ),
                  const Gap(5),
                  MacosIconButton(
                    icon: MacosIcon(
                      CupertinoIcons.plus_bubble,
                      color: MacosTheme.brightnessOf(context).resolve(
                        const Color.fromRGBO(0, 0, 0, 0.5),
                        const Color.fromRGBO(255, 255, 255, 0.5),
                      ),
                      size: 20.0,
                    ),
                    boxConstraints: BoxConstraints(),
                    onPressed: () {
                      setState(() {
                        history.clear();
                        _controller.clear();
                      });
                    },
                  ),
                  const Gap(15),
                  Text(
                    'Gemma 3',
                    style: TextStyle(fontSize: 15, fontWeight: FontWeight.w500),
                  ),
                  if (false) ...[
                    const Gap(6),
                    Container(
                      padding: const EdgeInsets.symmetric(
                        vertical: 2,
                        horizontal: 5,
                      ),
                      decoration: BoxDecoration(
                        color: Color.fromRGBO(127, 127, 127, 0.15),
                        borderRadius: BorderRadius.circular(5),
                      ),
                      child: Text(
                        '4B',
                        style: TextStyle(
                          fontSize: 11,
                          fontWeight: FontWeight.w600,
                          letterSpacing: 0.5,
                        ),
                      ),
                    ),
                  ],
                  const Gap(5),
                  MacosIcon(
                    CupertinoIcons.chevron_right,
                    size: 15,
                    color: MacosTheme.brightnessOf(context).resolve(
                      const Color.fromRGBO(0, 0, 0, 0.3),
                      const Color.fromRGBO(255, 255, 255, 0.3),
                    ),
                  ),
                ],
              ),
              actions: [
                ToolBarIconButton(
                  label: 'Share',
                  icon: MacosIcon(CupertinoIcons.share),
                  showLabel: false,
                  onPressed: () {},
                ),
                ToolBarIconButton(
                  label: 'Open in Companion Chat',
                  icon: MacosIcon(CupertinoIcons.square_on_square),
                  showLabel: false,
                  onPressed: () {},
                ),
              ],
            ),
            children: [
              ContentArea(
                builder: (
                  BuildContext context,
                  ScrollController scrollController,
                ) {
                  return ColoredBox(
                    color: MacosTheme.brightnessOf(
                      context,
                    ).resolve(const Color(0xFFFFFFFF), const Color(0xFF323232)),
                    child: Column(
                      children: [
                        Expanded(
                          child: ListView.separated(
                            cacheExtent: 10000,
                            reverse: true,
                            padding: const EdgeInsets.all(20),
                            itemBuilder: (context, index) {
                              final message = history.reversed.elementAt(index);
                              final out = message is HumanChatMessage;

                              final defaultTextStyle = MacosTheme.of(context)
                                  .typography
                                  .body
                                  .copyWith(fontSize: 14, height: 1.6);

                              if (out) {
                                return Align(
                                  alignment: Alignment.centerRight,
                                  child: ConstrainedBox(
                                    constraints: BoxConstraints(
                                      maxWidth: MediaQuery.of(context).size.width *
                                          0.6,
                                    ),
                                    child: Container(
                                      padding: const EdgeInsets.only(
                                        top: 4,
                                        bottom: 8,
                                        left: 14,
                                        right: 14,
                                      ),
                                      decoration: BoxDecoration(
                                        color: MacosTheme.brightnessOf(
                                          context,
                                        ).resolve(
                                          const Color(0xFFF2F2F2),
                                          const Color(0xFF4D4D4D),
                                        ),
                                        borderRadius: BorderRadius.circular(20),
                                      ),
                                      child: Text(
                                        message.contentAsString,
                                        style: defaultTextStyle,
                                      ),
                                    ),
                                  ),
                                );
                              }

                              return MarkdownBody(
                                selectable: true,
                                data: message.contentAsString,
                                styleSheet: MarkdownStyleSheet(
                                  p: defaultTextStyle,
                                  blockquote: defaultTextStyle,
                                  code: defaultTextStyle.copyWith(
                                    fontFamily: 'Menlo',
                                  ),
                                  codeblockDecoration: BoxDecoration(
                                    color: MacosTheme.brightnessOf(
                                      context,
                                    ).resolve(
                                      const Color(0xFFEEEEEE),
                                      const Color(0xFF3C3C3C),
                                    ),
                                    borderRadius: BorderRadius.circular(10),
                                  ),
                                  codeblockPadding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
                                  h1: MacosTheme.of(context).typography.title1,
                                  h2: MacosTheme.of(context).typography.title2,
                                  h3: MacosTheme.of(context).typography.title3,
                                  listBullet: defaultTextStyle,
                                ),
                              );
                            },
                            separatorBuilder: (context, index) => const Gap(30),
                            itemCount: history.length,
                          ),
                        ),
                        Container(
                          margin: const EdgeInsets.only(
                            bottom: 16,
                            left: 16,
                            right: 16,
                          ),
                          decoration: BoxDecoration(
                            border: Border.all(
                              color: MacosTheme.brightnessOf(context).resolve(
                                const Color(0xFFDADADA),
                                const Color(0xFF505050),
                              ),
                            ),
                            color: MacosTheme.brightnessOf(context).resolve(
                              const Color(0xFFF2F2F2),
                              const Color(0xFF3C3C3C),
                            ),
                            borderRadius: BorderRadius.circular(16),
                          ),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.stretch,
                            children: [
                              MacosTextField.borderless(
                                autofocus: true,
                                controller: _controller,
                                padding: EdgeInsets.only(left: 14, right: 14, top: 14, bottom: 12),
                                style: TextStyle(fontSize: 14),
                                placeholder: 'Ask anything',
                                placeholderStyle: TextStyle(
                                  color: MacosTheme.brightnessOf(
                                    context,
                                  ).resolve(
                                    Color.fromRGBO(0, 0, 0, 0.5),
                                    Color.fromRGBO(255, 255, 255, 0.5),
                                  ),
                                ),
                                onSubmitted: (value) async {
                                  if (value.isEmpty) {
                                    return;
                                  }

                                  final message = HumanChatMessage(
                                    content: ChatMessageContent.text(value),
                                  );

                                  _controller.clear();

                                  setState(() {
                                    history.add(message);
                                  });

                                  final prompt = ChatPromptTemplate.fromPromptMessages([
                                    SystemChatMessagePromptTemplate.fromTemplate(
                                      'You are a helpful assistant who responds with messages no longer than 6 sentences. You love using Markdown formatting to ensure clarity and readability. You hate emojis.',
                                    ),
                                    MessagesPlaceholder(
                                      variableName: 'chat_history',
                                    ),
                                  ]);

                                  final chain = prompt.pipe(chat);

                                  final response = chain.stream({
                                    'chat_history': history,
                                  });

                                  history.add(AIChatMessage(content: ''));

                                  await for (final part in response) {
                                    if (history.isEmpty) {
                                      return;
                                    }

                                    history.last = AIChatMessage(
                                      content:
                                          history.last.contentAsString +
                                          part.outputAsString,
                                    );
                                    setState(() {});
                                  }
                                },
                              ),
                              Padding(
                                padding: const EdgeInsets.only(bottom: 10, left: 14, right: 10),
                                child: Row(
                                  children: [
                                    MacosIcon(
                                      CupertinoIcons.plus,
                                      size: 20,
                                      color: MacosTheme.brightnessOf(
                                        context,
                                      ).resolve(
                                        const Color.fromRGBO(0, 0, 0, 0.5),
                                        const Color.fromRGBO(255, 255, 255, 0.5),
                                      ),
                                    ),
                                    const Gap(14),
                                    MacosIcon(
                                      CupertinoIcons.globe,
                                      size: 20,
                                      color: MacosTheme.brightnessOf(
                                        context,
                                      ).resolve(
                                        const Color.fromRGBO(0, 0, 0, 0.5),
                                        const Color.fromRGBO(255, 255, 255, 0.5),
                                      ),
                                    ),
                                    const Gap(14),
                                    MacosIcon(
                                      CupertinoIcons.cursor_rays,
                                      size: 20,
                                      color: MacosTheme.brightnessOf(
                                        context,
                                      ).resolve(
                                        const Color.fromRGBO(0, 0, 0, 0.5),
                                        const Color.fromRGBO(255, 255, 255, 0.5),
                                      ),
                                    ),
                                    const Spacer(),
                                    AnimatedBuilder(
                                      animation: _controller,
                                      builder: (context, child) {
                                        return AnimatedSwitcher(
                                          switchInCurve: Curves.easeOut,
                                          switchOutCurve: Curves.easeIn,
                                          duration: const Duration(
                                            milliseconds: 150,
                                          ),
                                          transitionBuilder: (child, animation) {
                                            return FadeTransition(
                                              opacity: animation,
                                              child: ScaleTransition(
                                                scale: Tween(
                                                  begin: 0.0,
                                                  end: 1.0,
                                                ).animate(animation),
                                                child: child,
                                              ),
                                            );
                                          },
                                          child: Container(
                                            key: ValueKey(
                                              _controller.text.isEmpty,
                                            ),
                                            alignment: Alignment.center,
                                            constraints: BoxConstraints.tight(
                                              Size.square(32),
                                            ),
                                            decoration: BoxDecoration(
                                              color: MacosTheme.brightnessOf(
                                                context,
                                              ).resolve(
                                                MacosColors.black,
                                                MacosColors.white,
                                              ),
                                              shape: BoxShape.circle,
                                            ),
                                            child: Icon(
                                              _controller.text.isEmpty
                                                  ? CupertinoIcons.mic_fill
                                                  : CupertinoIcons.arrow_up,
                                              size: 20,
                                              color: MacosTheme.brightnessOf(
                                                context,
                                              ).resolve(
                                                MacosColors.white,
                                                MacosColors.black,
                                              ),
                                            ),
                                          ),
                                        );
                                      },
                                    ),
                                  ],
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  );
                },
              ),
            ],
          );
        },
      ),
    );
  }
}
